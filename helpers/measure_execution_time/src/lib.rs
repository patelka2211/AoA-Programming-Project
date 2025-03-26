use std::time::Instant;

use data_types::TimeMeasuredOutput;

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
