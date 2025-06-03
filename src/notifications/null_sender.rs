use crate::entities::Notifier;

pub struct NullSender;

impl Notifier for NullSender {
    fn message(&self, message: String) -> anyhow::Result<()> {
        println!("{message}");
        Ok(())
    }
}
