use crate::stack_frame::StackFrame;

use crate::value::value::StackFrameValue;
extern crate log;
extern crate env_logger;



use crate::u8c::u8s_to_u16;
pub fn bipush(frame: &mut StackFrame) {
    let u: u8 = frame.code[frame.pc + 1];
    frame.op_stack.push(StackFrameValue::Byte(u as i8));
    frame.pc += 2;
}

pub fn sipush(frame: &mut StackFrame) {
    //info!("{:?}",frame);
    let v = u8s_to_u16(&frame.code[frame.pc + 1..frame.pc + 3]) as i16;
    frame
        .op_stack
        .push(StackFrameValue::Short(v));
    frame.pc += 3;
}