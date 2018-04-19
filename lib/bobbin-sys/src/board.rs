pub trait Board {
    type System;
    fn id(&self) -> &'static str;
    fn sys(&self) -> &Self::System;
    fn sys_mut(&mut self) -> &mut Self::System;
}