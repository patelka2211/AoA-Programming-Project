use std::time::{Duration, Instant};

pub struct TimeMeasuredOutput<T> {
    pub duration: Duration,
    pub output: T,
}

pub fn measure_execution_time<FunctionType, ArgsType, ReturnType>(
    function: FunctionType,
    args: ArgsType,
) -> TimeMeasuredOutput<ReturnType>
where
    FunctionType: Fn(ArgsType) -> ReturnType,
{
    let start = Instant::now();
    let output = function(args);
    let duration = start.elapsed();

    TimeMeasuredOutput { duration, output }
}
