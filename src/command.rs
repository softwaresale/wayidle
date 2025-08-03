use std::process::Command;

use log::{info, warn};

use crate::err::WayIdleResult;

pub fn execute_command(command_line: &[String]) -> WayIdleResult<()> {
    if command_line.is_empty() {
        warn!("empty command line provided, returning early");
        return Ok(());
    }

    info!("executing command {:?}", command_line);

    let program = command_line.first().unwrap();

    let command_args = command_line.iter().skip(1);
    let child = Command::new(program).args(command_args).spawn()?;

    info!("executed child process {}", child.id());

    Ok(())
}
