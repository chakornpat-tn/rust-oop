trait Notifier {
    fn notify(&self, message: &str);
    
} 

struct DMSNotifier;

impl Notifier for DMSNotifier {
    fn notify(&self, message: &str) {
        println!("DMS Notification: {}", message);
    }
}

struct EmailNotifier;

impl Notifier for EmailNotifier {
    fn notify(&self, message: &str) {
        println!("Email Notification: {}", message);
    }
}

struct NotifierManager;
impl NotifierManager {
    fn send_notifier<T: Notifier>(&self, notifier: T) {
        notifier.notify("completed");
    }
}

fn main() {
    let notify_manager = NotifierManager;

    let dms_notifier = DMSNotifier;
    let email_notifier = EmailNotifier;

    notify_manager.send_notifier(dms_notifier);
    notify_manager.send_notifier(email_notifier);

}
