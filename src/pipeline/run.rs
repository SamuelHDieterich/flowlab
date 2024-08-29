//----------------//
//---  CRATES  ---//
//----------------//

// Internal modules
use crate::{
    device::{Arguments, Device, Query},
    instruction::Instruction,
    pipeline::{DeviceInstruction, Pipeline, Scan, ScanType, Step, WaitFor},
    Data,
};

// Built-in modules
//// Standard library
use std::{
    collections::{BTreeMap, HashMap},
    path::PathBuf,
    sync::{Arc, Mutex},
};

// External crates
//// Serde: Serialization/Deserialization framework
use serde::de::DeserializeOwned;
//// Tokio: Asynchronous runtime
use tokio::task::JoinSet;
use tokio::time::{Duration, Instant};
//// Templating engine
use tera;
//// CSV: Reading and writing CSV files
use csv;
//// Time
use chrono;

//------------------------//
//---  IMPLEMENATIONS  ---//
//------------------------//

impl<Protocol> Pipeline<Protocol>
where
    Protocol:
        DeserializeOwned + Query + std::fmt::Debug + std::clone::Clone + Send + Sync + 'static,
{
    /// Execute the pipeline.
    #[tracing::instrument(name = "Pipeline::execute", skip(self))]
    pub fn execute(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Create a parameters stack
        let mut parameters_stack = BTreeMap::new();

        // Include some constant values to the parameters stack
        // 1. Include the pipeline name
        parameters_stack.insert(
            "PIPELINE_NAME".to_string(),
            Arguments::new("PIPELINE_NAME".to_string(), Data::String(self.name.clone())),
        );
        // 2. Include the pipeline description
        parameters_stack.insert(
            "PIPELINE_DESCRIPTION".to_string(),
            Arguments::new(
                "PIPELINE_DESCRIPTION".to_string(),
                Data::String(self.description.clone()),
            ),
        );
        // 3. Include the pipeline start time
        let pipeline_start_time = chrono::Utc::now();
        parameters_stack.insert(
            "PIPELINE_START_TIME".to_string(),
            Arguments::new(
                "PIPELINE_START_TIME".to_string(),
                Data::String(pipeline_start_time.format("%Y-%m-%d_%H-%M-%S").to_string()),
            ),
        );

        // Pipeline data
        tracing::info!("Executing pipeline: {}", self.name);
        let pipeline_length = self.pipeline.len();

        // Iterate over the steps in the pipeline
        for (index, step) in self.pipeline.iter().enumerate() {
            let step_time = chrono::Utc::now();
            let step_name = match step {
                Step::Instruction(instruction) => &instruction.instruction,
                Step::WaitFor(_) => "Wait for",
                Step::Scan(_) => "Scan",
            };
            tracing::info!(
                "Executing step {}/{} - {}",
                index + 1,
                pipeline_length,
                step_name
            );
            // 4. Include the step index
            parameters_stack.insert(
                "STEP_INDEX".to_string(),
                Arguments::new("STEP_INDEX".to_string(), Data::Integer(index as i64)),
            );
            // Execute the step
            let data = Pipeline::_execute(&self.devices, &step, &parameters_stack)?;
            if let Some(data) = data {
                tracing::info!("Data: {:#?}", data);
            }
            tracing::info!(
                "Step completed. Time elapsed: {:?}",
                (chrono::Utc::now() - step_time)
                    .to_std()
                    .unwrap_or(Duration::from_secs(0))
            );
        }
        tracing::info!(
            "Pipeline execution completed. Time elapsed: {:?}",
            (chrono::Utc::now() - pipeline_start_time)
                .to_std()
                .unwrap_or(Duration::from_secs(0))
        );

        Ok(())
    }

    fn _execute<'a>(
        devices: &'a HashMap<String, Device<Protocol>>,
        step: &Step,
        _parameters_stack: &BTreeMap<String, Arguments>,
    ) -> Result<
        Option<BTreeMap<String, BTreeMap<String, Data>>>,
        Box<dyn std::error::Error + Send + Sync>,
    > {
        match step {
            Step::Instruction(device_instruction) => {
                // Get the device
                let device = device_instruction.get_device(devices)?;

                // Get the instruction
                let instruction = device_instruction.get_instruction(device)?;

                // Merge the parameters
                let parameters = device_instruction.merge_parameters(device, _parameters_stack);

                // Render the query
                let query = device_instruction.render_query(device, &parameters)?;

                // Create a runtime to run async code
                let runtime = tokio::runtime::Builder::new_multi_thread()
                    .worker_threads(1)
                    .enable_all()
                    .build()?;

                // Execute the instruction
                let data =
                    runtime.block_on(device_instruction.execute(device, instruction, &query))?;

                Ok(data)
            }
            Step::WaitFor(wait_for) => {
                match &wait_for.metric {
                    // Complex case: wait for a metric to meet a condition
                    Some(metric) => {
                        // Get the device
                        let device = metric.get_device(devices)?;

                        // Get the instruction
                        let instruction = metric.get_instruction(device)?;

                        // Merge the parameters
                        let parameters = metric.merge_parameters(device, _parameters_stack);

                        // Render the query
                        let query = metric.render_query(device, &parameters)?;

                        // If inside a Scan, target should already be defined in parameters
                        let mut target = None;
                        if wait_for.parameters.name.is_some() & wait_for.parameters.value.is_none()
                        {
                            let key = wait_for.parameters.name.clone().unwrap();
                            let arg = parameters.get(&key);
                            target = match arg {
                                Some(value) => Some(value.value.clone()),
                                _ => None,
                            }
                        }

                        // Create a runtime to run async code
                        let runtime = tokio::runtime::Builder::new_multi_thread()
                            .worker_threads(1)
                            .enable_all()
                            .build()?;

                        let _: Result<(), Box<dyn std::error::Error + Send + Sync>> = runtime
                            .block_on(async {
                                // Variables used in the loop
                                let mut interval =
                                    tokio::time::interval(Duration::from_millis(100)); // TODO: Make this configurable
                                let mut timer: Option<Instant> = None;

                                // Loop until the condition is met
                                loop {
                                    // Wait for the interval
                                    interval.tick().await;
                                    // Execute the instruction
                                    match metric
                                        .execute(device, instruction, &query)
                                        .await
                                        .map_err(|e| {
                                            tracing::error!("Error executing instruction: {}", e);
                                            e
                                        })? {
                                        // Check the condition
                                        Some(data) => {
                                            if wait_for.check_condition(
                                                &data.get(&device.name).unwrap(),
                                                &target,
                                            ) {
                                                // Start the timer
                                                if timer.is_none() {
                                                    tracing::info!(
                                                        "Condition met. Starting the delay timer."
                                                    );
                                                    timer = Some(Instant::now());
                                                }
                                            } else {
                                                if timer.is_some() {
                                                    tracing::info!(
                                                    "Condition not met. Resetting the delay timer."
                                                );
                                                    timer = None;
                                                }
                                            }
                                        }
                                        // No data returned
                                        None => {
                                            // Start the timer
                                            if timer.is_none() {
                                                tracing::error!(
                                                "No data returned. Falling back to the delay time."
                                            );
                                                timer = Some(Instant::now());
                                            }
                                        }
                                    };
                                    // Check if the timer has elapsed
                                    if let Some(timer) = timer {
                                        if timer.elapsed()
                                            >= Duration::from_secs(wait_for.parameters.delay)
                                        {
                                            tracing::info!(
                                                "Delay time elapsed. Continuing the pipeline."
                                            );
                                            break;
                                        }
                                    }
                                }
                                Ok(())
                            });
                        Ok(None)
                    }
                    // Simple case: wait for a fixed amount of time
                    None => {
                        tracing::info!("No metric defined. Using just the delay time");
                        std::thread::sleep(Duration::from_secs(wait_for.parameters.delay));
                        tracing::info!("Delay time elapsed. Continuing the pipeline.");
                        return Ok(None);
                    }
                }
            }
            Step::Scan(scan) => {
                // Check the scan type just once instead of checking it in every loop
                match &scan.scan_type {
                    ScanType::Settle => {
                        // Prepare the datafile
                        let datafile = scan.create_datafile(_parameters_stack)?;
                        let file = match datafile {
                            Some(datafile) => {
                                tracing::info!("Creating datafile: {}", datafile.display());
                                match std::fs::File::create(&datafile) {
                                    Ok(file) => Some(file),
                                    Err(e) => {
                                        tracing::error!("Error creating datafile: {}", e);
                                        None
                                    }
                                }
                            }
                            None => {
                                tracing::info!(
                                    "No datafile defined. Skipping the datafile creation."
                                );
                                None
                            }
                        };
                        let mut writer = match file {
                            Some(file) => Some(csv::Writer::from_writer(file)),
                            None => None,
                        };

                        // Given that the scan loop can accept floating point numbers, we need to use a while loop
                        let mut variable_parameter = scan.parameters.start;
                        let sign = scan.parameters.stop > scan.parameters.start;
                        while if sign {
                            variable_parameter <= scan.parameters.stop
                        } else {
                            variable_parameter >= scan.parameters.stop
                        } {
                            tracing::info!(variable_parameter);

                            // Update the parameters stack
                            let mut parameters_stack = _parameters_stack.clone();
                            parameters_stack.insert(
                                scan.parameters.variable.clone(),
                                Arguments::new(
                                    scan.parameters.variable.clone(),
                                    Data::Float(variable_parameter),
                                ),
                            );

                            // Create a runtime to run async code
                            let runtime = tokio::runtime::Builder::new_multi_thread()
                                .worker_threads(4)
                                .enable_all()
                                .build()?;

                            // Execute the scan metrics
                            for metric in &scan.metrics {
                                Pipeline::_execute(devices, metric, &parameters_stack)?;
                            }

                            // Response data
                            let mut data = BTreeMap::new();
                            let mut temp_map = BTreeMap::new();
                            temp_map.insert(
                                "Datetime".to_string(),
                                Data::String(chrono::Utc::now().to_rfc3339()),
                            );
                            temp_map.insert(
                                scan.parameters.variable.clone(),
                                Data::Float(variable_parameter),
                            );
                            data.insert("".to_string(), temp_map);

                            // Execute the scan measures
                            // let measure_results = runtime
                            //     .block_on(scan.execute_measures(devices, &parameters_stack))?;
                            // let devices = Arc::new(devices.clone());
                            // let parameters_stack = Arc::new(parameters_stack.clone());
                            // let measure_handle = tokio::spawn(async move {
                            //     scan.execute_measures(devices, &parameters_stack).await
                            // });
                            // let measure_results = runtime.block_on(measure_handle)??;

                            // data.extend(measure_results);

                            tracing::info!("{:#?}", data);

                            // Write the data to the file
                            match &mut writer {
                                Some(writer) => {
                                    writer.serialize(data).unwrap_or_else(|e| {
                                        tracing::error!("Error writing data to the file: {}", e);
                                    });
                                }
                                None => {
                                    tracing::debug!("Data: {:#?}", data);
                                }
                            }

                            // Next step
                            variable_parameter += scan.parameters.step;
                        }
                    }
                    ScanType::Sweep => {
                        // Prepare the datafile
                        let datafile = scan.create_datafile(_parameters_stack)?;
                        let writer = match datafile {
                            Some(file) => Some(Arc::new(Mutex::new(csv::Writer::from_path(file)?))),
                            None => None,
                        };

                        // Given that the scan loop can accept floating point numbers, we need to use a while loop
                        let mut variable_parameter = scan.parameters.start;
                        let sign = scan.parameters.stop > scan.parameters.start;
                        while if sign {
                            variable_parameter <= scan.parameters.stop
                        } else {
                            variable_parameter >= scan.parameters.stop
                        } {
                            // Update the parameters stack
                            let mut parameters_stack = _parameters_stack.clone();
                            parameters_stack.insert(
                                scan.parameters.variable.clone(),
                                Arguments::new(
                                    scan.parameters.variable.clone(),
                                    Data::Float(variable_parameter),
                                ),
                            );

                            // Create a runtime to run async code
                            let runtime = tokio::runtime::Builder::new_multi_thread()
                                .worker_threads(1)
                                .enable_all()
                                .build()?;

                            // Create a metric flag to notify if the metric loop is done
                            let is_metric_done = Arc::new(Mutex::new(false));

                            // Create Arcs for the devices and parameters stack
                            let devices = Arc::new(devices.clone());
                            let scan = Arc::new(scan.clone());
                            let parameters_stack = Arc::new(parameters_stack.clone());

                            // Spawn a metric loop
                            let metric_loop = {
                                let devices = devices.clone();
                                let scan = scan.clone();
                                let parameters_stack = parameters_stack.clone();
                                let is_metric_done = is_metric_done.clone();
                                async move {
                                    // Execute the scan metrics
                                    for metric in &scan.metrics {
                                        Pipeline::_execute(&devices, metric, &parameters_stack)?;
                                    }
                                    // Set the metric flag
                                    match is_metric_done.lock() {
                                        Ok(mut is_metric_done) => {
                                            *is_metric_done = true;
                                        }
                                        Err(e) => {
                                            tracing::error!("Error locking the metric flag: {}", e);
                                        }
                                    }
                                    Ok(()) as Result<(), Box<dyn std::error::Error + Send + Sync>>
                                }
                            };

                            // Spawn a measure loop
                            let measure_loop = {
                                let devices = devices.clone();
                                let scan = scan.clone();
                                let parameters_stack = parameters_stack.clone();
                                let is_metric_done = is_metric_done.clone();
                                let writer = writer.clone();

                                async move {
                                    // Variables used in the loop
                                    let mut interval =
                                        tokio::time::interval(Duration::from_millis(100)); // TODO: Make this configurable

                                    // Loop until the condition is met
                                    loop {
                                        // Wait for the interval
                                        interval.tick().await;

                                        // Execute the scan measures
                                        let measure_results = scan
                                            .execute_measures(&devices, &parameters_stack)
                                            .await
                                            .map_err(|e| {
                                                tracing::error!("Error executing measures: {}", e);
                                                e
                                            })?;

                                        // Response data
                                        let mut data = BTreeMap::new();
                                        let mut temp_map = BTreeMap::new();
                                        temp_map.insert(
                                            "Datetime".to_string(),
                                            Data::String(chrono::Utc::now().to_rfc3339()),
                                        );
                                        temp_map.insert(
                                            scan.parameters.variable.clone(),
                                            Data::Float(variable_parameter),
                                        );
                                        data.insert("".to_string(), temp_map);
                                        data.extend(measure_results);

                                        // Write the data to the file
                                        if let Some(writer) = &writer {
                                            let mut writer = writer
                                                .lock()
                                                .map_err(|e| {
                                                    tracing::error!(
                                                        "Error locking the writer: {}",
                                                        e
                                                    );
                                                    e
                                                })
                                                .unwrap();
                                            writer.serialize(data).map_err(|e| {
                                                tracing::error!(
                                                    "Error writing data to the file: {}",
                                                    e
                                                );
                                                e
                                            })?;
                                        } else {
                                            tracing::debug!("Data: {:#?}", data);
                                        }

                                        // Check if the metric loop is done
                                        match is_metric_done.lock() {
                                            Ok(is_metric_done) => {
                                                if *is_metric_done {
                                                    break;
                                                }
                                            }
                                            Err(e) => {
                                                tracing::error!(
                                                    "Error locking the metric flag: {}",
                                                    e
                                                );
                                            }
                                        }
                                    }
                                    Ok(()) as Result<(), Box<dyn std::error::Error + Send + Sync>>
                                }
                            };

                            // Instantiate the tasks
                            let metric_loop = tokio::task::spawn(metric_loop);
                            let measure_loop = tokio::task::spawn(measure_loop);

                            // Wait for the tasks to finish
                            let tasks_result = runtime
                                .block_on(async { tokio::try_join!(metric_loop, measure_loop) });
                            match tasks_result {
                                Ok(_) => {
                                    tracing::trace!("Tasks completed successfully");
                                }
                                Err(e) => {
                                    tracing::error!("Error executing tasks: {}", e);
                                }
                            }

                            // Next step
                            variable_parameter += scan.parameters.step;
                        }
                    }
                }
                Ok(None)
            }
        }
    }
}

impl DeviceInstruction {
    #[tracing::instrument(name = "DeviceInstruction::get_device", level = "debug", skip(self))]
    fn get_device<'a, Protocol>(
        &'a self,
        devices: &'a HashMap<String, Device<Protocol>>,
    ) -> Result<&'a Device<Protocol>, Box<dyn std::error::Error + Send + Sync>>
    where
        Protocol: DeserializeOwned + Query + std::fmt::Debug + std::clone::Clone,
    {
        let device = devices.get(&self.device).ok_or_else(|| {
            tracing::error!("Device not found");
            "Device not found"
        })?;
        tracing::debug!(?device);
        Ok(device)
    }

    #[tracing::instrument(
        name = "DeviceInstruction::merge_parameters",
        level = "debug",
        skip(self, device)
    )]
    fn merge_parameters<Protocol>(
        &self,
        device: &Device<Protocol>,
        parameters_stack: &BTreeMap<String, Arguments>,
    ) -> BTreeMap<String, Arguments>
    where
        Protocol: DeserializeOwned + Query + std::fmt::Debug + std::clone::Clone,
    {
        // Get the parameters to format the query
        // 1. Get the default arguments from the device
        let mut parameters = device.default_arguments.clone();
        // 2. Add the stack
        parameters.extend(parameters_stack.clone());
        // 3. Add the instruction arguments
        parameters.extend(self.parameters.clone());
        tracing::debug!(?parameters);

        parameters
    }

    #[tracing::instrument(
        name = "DeviceInstruction::get_instruction",
        level = "debug",
        skip(self, device)
    )]
    fn get_instruction<'a, Protocol>(
        &'a self,
        device: &'a Device<Protocol>,
    ) -> Result<&'a Instruction, Box<dyn std::error::Error + Send + Sync>>
    where
        Protocol: DeserializeOwned + Query + std::fmt::Debug + std::clone::Clone,
    {
        let instruction = device.instructions.get(&self.instruction).ok_or_else(|| {
            tracing::error!("Instruction not found");
            "Instruction not found"
        })?;
        tracing::debug!(?instruction);

        Ok(instruction)
    }

    #[tracing::instrument(
        name = "DeviceInstruction::render_query",
        level = "debug",
        skip(self, device, parameters)
    )]
    fn render_query<Protocol>(
        &self,
        device: &Device<Protocol>,
        parameters: &BTreeMap<String, Arguments>,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>
    where
        Protocol: DeserializeOwned + Query + std::fmt::Debug + std::clone::Clone,
    {
        // Get the instruction
        let instruction = self.get_instruction(device)?;

        // Render the query
        let query = instruction.command.render(&parameters)?;
        tracing::debug!(?query);

        Ok(query)
    }

    #[tracing::instrument(
        name = "DeviceInstruction::execute",
        level = "debug",
        skip(self, device, instruction)
    )]
    async fn execute<Protocol>(
        &self,
        device: &Device<Protocol>,
        instruction: &Instruction,
        query: &str,
    ) -> Result<
        Option<BTreeMap<String, BTreeMap<String, Data>>>,
        Box<dyn std::error::Error + Send + Sync>,
    >
    where
        Protocol: DeserializeOwned + Query + std::fmt::Debug + std::clone::Clone,
    {
        // Query the device
        let response = device.protocol.query(&query).await?;
        tracing::debug!(?response);

        // Parse the response
        // Only extract the response from instructions that are set for it
        // This can avoid problems with instructions that return just an ACK
        if instruction.response.is_some() {
            let data = match response {
                Some(response) => {
                    let response = instruction
                        .response
                        .as_ref()
                        .ok_or_else(|| {
                            tracing::error!("Instruction was not expecting a response");
                            "Instruction was not expecting a response"
                        })?
                        .parse(&response)?;
                    let mut data = BTreeMap::new();
                    data.insert(self.device.clone(), response);
                    Some(data)
                }
                None => None,
            };
            tracing::debug!(?data);
            Ok(data)
        } else {
            Ok(None)
        }
    }
}

impl WaitFor {
    #[tracing::instrument(name = "WaitFor::check_condition", level = "debug")]
    fn check_condition(
        &self,
        data: &BTreeMap<String, Data>,
        external_target: &Option<Data>,
    ) -> bool {
        // Get the value name
        let attribute = match &self.parameters.name {
            Some(name) => name,
            None => {
                tracing::warn!("Parameter name not defined. Skipping the condition check.");
                return true;
            }
        };
        // Get the value
        let value = match data.get(attribute) {
            Some(value) => value,
            None => {
                tracing::error!("Attribute not found. Skipping the condition check.");
                return true;
            }
        };
        // Get the target value
        let target: f64;
        match external_target {
            Some(Data::Float(value)) => target = *value,
            Some(Data::Integer(value)) => target = *value as f64,
            _ => {
                target = match self.parameters.value {
                    Some(value) => value,
                    None => {
                        tracing::error!("Target value not defined. Skipping the condition check.");
                        return true;
                    }
                };
            }
        }
        // Get the acceptable tolerance/threshold value
        let tolerance = match self.parameters.tolerance {
            Some(tolerance) => tolerance,
            None => {
                tracing::error!("Tolerance not defined. Using 0 as the default value.");
                0.0
            }
        };
        // Check the condition
        match value {
            Data::Integer(value) => {
                if (*value as f64 - target).abs() <= tolerance {
                    tracing::debug!(%value, %target, %tolerance, "Condition met: |{} - {}| <= {}", value, target, tolerance);
                    true
                } else {
                    tracing::debug!(%value, %target, %tolerance, "Condition not met: |{} - {}| > {}", value, target, tolerance);
                    false
                }
            }
            Data::Float(value) => {
                if (value - target).abs() <= tolerance {
                    tracing::debug!(%value, %target, %tolerance, "Condition met: |{} - {}| <= {}", value, target, tolerance);
                    true
                } else {
                    tracing::debug!(%value, %target, %tolerance, "Condition not met: |{} - {}| > {}", value, target, tolerance);
                    false
                }
            }
            _ => {
                tracing::error!("Invalid data type. Skipping the condition check.");
                true
            }
        }
    }
}

impl Scan {
    #[tracing::instrument(name = "Scan::create_datafile", level = "debug")]
    fn create_datafile(
        &self,
        parameters: &BTreeMap<String, Arguments>,
    ) -> Result<Option<PathBuf>, Box<dyn std::error::Error + Send + Sync>> {
        // Get the template datafile name
        let template = match &self.datafile {
            Some(datafile) => datafile,
            None => {
                tracing::info!("Datafile template not defined. Skipping the datafile creation.");
                return Ok(None);
            }
        };

        // Convert the parameters to a context
        let mut context = tera::Context::new();
        for (name, arguments) in parameters {
            context.insert(name, &arguments.value.to_string());
        }
        // Create a Tera instance
        let mut tera = tera::Tera::default();
        tera.add_raw_template("datafile", template).map_err(|e| {
            tracing::error!("Error adding datafile template: {}", e);
            e
        })?;
        // Render the datafile
        let rendered_datafile = tera.render("datafile", &context).map_err(|e| {
            tracing::error!("Error rendering datafile: {}", e);
            e
        })?;
        tracing::debug!(?rendered_datafile);

        Ok(Some(PathBuf::from(rendered_datafile)))
    }

    #[tracing::instrument(name = "Scan::execute_measures", level = "debug")]
    async fn execute_measures<Protocol>(
        &self,
        devices: &HashMap<String, Device<Protocol>>,
        parameters: &BTreeMap<String, Arguments>,
    ) -> Result<BTreeMap<String, BTreeMap<String, Data>>, Box<dyn std::error::Error + Send + Sync>>
    where
        Protocol:
            DeserializeOwned + Query + std::fmt::Debug + std::clone::Clone + Send + Sync + 'static,
    {
        // Create a join set - a set of futures that can be joined
        let mut futures = JoinSet::new();

        // Execute the measures
        tracing::trace!("Executing measures");
        for measure in &self.measures {
            // Clone the variables
            // TODO: Find a better way to clone the variables - Rc and Arc
            let devices = devices.clone();
            let measure = measure.clone();
            let parameters = parameters.clone();

            tracing::trace!("Spawning measure");
            futures.spawn(async move {
                let response = Pipeline::_execute(&devices, &measure, &parameters);
                response
            });
        }

        // Collect the data
        let mut data = BTreeMap::new();
        tracing::trace!("Collecting data");
        while let Some(response) = futures.join_next().await {
            let response = response??;
            if let Some(response) = response {
                data.extend(response);
            }
        }
        tracing::debug!(?data);
        Ok(data)
    }
}
