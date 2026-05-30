use std::collections::HashMap;

pub fn ios_usage_keys() -> HashMap<String, String> {
    HashMap::from([
        (
            "NSCameraUsageDescription".into(),
            "Camera".into(),
        ),
        (
            "NSMicrophoneUsageDescription".into(),
            "Microphone".into(),
        ),
        (
            "NSLocationWhenInUseUsageDescription".into(),
            "Location (When In Use)".into(),
        ),
        (
            "NSLocationAlwaysUsageDescription".into(),
            "Location (Always)".into(),
        ),
        (
            "NSLocationAlwaysAndWhenInUseUsageDescription".into(),
            "Location".into(),
        ),
        (
            "NSPhotoLibraryUsageDescription".into(),
            "Photo Library".into(),
        ),
        (
            "NSPhotoLibraryAddUsageDescription".into(),
            "Photo Library (Add)".into(),
        ),
        (
            "NSContactsUsageDescription".into(),
            "Contacts".into(),
        ),
        (
            "NSCalendarsUsageDescription".into(),
            "Calendar".into(),
        ),
        (
            "NSBluetoothAlwaysUsageDescription".into(),
            "Bluetooth".into(),
        ),
        (
            "NSHealthShareUsageDescription".into(),
            "HealthKit (Read)".into(),
        ),
        (
            "NSHealthUpdateUsageDescription".into(),
            "HealthKit (Write)".into(),
        ),
        (
            "NSUserTrackingUsageDescription".into(),
            "User Tracking".into(),
        ),
        (
            "NSFaceIDUsageDescription".into(),
            "Face ID".into(),
        ),
        (
            "NSMotionUsageDescription".into(),
            "Motion".into(),
        ),
        (
            "NSRemindersUsageDescription".into(),
            "Reminders".into(),
        ),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_camera_key() {
        let keys = ios_usage_keys();
        assert_eq!(
            keys.get("NSCameraUsageDescription"),
            Some(&"Camera".to_string())
        );
    }
}
