use crate::pool::Job;

pub(crate) enum Message {
    NewJob(Job),
    Terminate
}