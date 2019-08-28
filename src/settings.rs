use console::Term;
use indexmap::IndexMap;
use dialoguer::{
    Checkboxes,
    Confirmation,
    Select
};

use crate::main;
use crate::utils::{
    get_theme,
    update_file
};


fn restart() {
    if Confirmation::with_theme(get_theme())
        .with_text("Do you want to search for another setting?")
        .interact()
        .expect("Confirmation could not be accepted.") {
            microsoft_settings_menu();
    } else {
        Term::stdout()
            .clear_screen()
            .expect("Terminal could not be cleared.");

        main();
    }
}

pub fn microsoft_settings_menu() {
    Term::stdout()
        .clear_screen()
        .expect("Line could not be cleared.");

    // Microsoft Settings Menu
    let items = &[
        "Accounts",
        "Apps",
        "Cortana",
        "Devices",
        "Ease of Access",
        "Extras",
        "Gaming",
        "Home",
        "Mixed Reality",
        "Network and Internet",
        "Personalization",
        "Phone",
        "Privacy",
        "Suface Hub",
        "System",
        "Time and Language",
        "Update and Security",
        "User Accounts",
    ];

    // https://docs.microsoft.com/en-us/windows/uwp/launch-resume/launch-settings-app
    let accounts: IndexMap<&str, &str> = [
        ("Access work or school", "start ms-settings:workplace"),
        ("Email and app accounts", "start ms-settings:emailandaccounts"),
        ("Family and other people", "start ms-settings:otherusers"),
        ("Set up a kiosk", "start ms-settings:assignedaccess"),
        ("Sign-in options ", "start ms-settings:signinoptions"),
        ("Sync your settings", "start ms-settings:sync"),
        ("Windows Hello setup", "start ms-settings:signinoptions-launchfaceenrollment"),
        ("Your info", "start ms-settings:yourinfo"),
    ].iter().cloned().collect();

    let apps: IndexMap<&str, &str> = [
        ("Apps and Features", "start ms-settings:appsfeatures"),
        ("App features", "start ms-settings:appsfeatures-app"),
        ("Apps for websites", "start ms-settings:appsforwebsites"),
        ("Default apps", "start ms-settings:defaultapps"),
        ("Manage optional features", "start ms-settings:optionalfeatures"),
        ("Offline Maps", "start ms-settings:maps"),
        ("Startup apps", "start ms-settings:startupapps"),
        ("Video playback", "start ms-settings:videoplayback"),
    ].iter().cloned().collect();

    let cortana: IndexMap<&str, &str> = [
        ("Cortana across my devices", "start ms-settings:cortana-notifications"),
        ("More details", "start ms-settings:cortana-moredetails"),
        ("Permissions and History", "start ms-settings:cortana-permissions"),
        ("Searching Windows", "start ms-settings:cortana-windowssearch"),
        ("Talk to Cortana", "start ms-settings:cortana"),
    ].iter().cloned().collect();

    let devices: IndexMap<&str, &str> = [
        ("AutoPlay", "start ms-settings:autoplay"),
        ("Bluetooth", "start ms-settings:bluetooth"),
        ("Connected Devices", "start ms-settings:connecteddevices"),
        ("Mouse and touchpad", "start ms-settings:mousetouchpad"),
        ("Pen and Windows Ink", "start ms-settings:pen"),
        ("Printers and scanners", "start ms-settings:printers"),
        ("Touchpad", "start ms-settings:devices-touchpad"),
        ("Typing", "start ms-settings:typing"),
        ("USB", "start ms-settings:usb"),
        ("Wheel", "start ms-settings:wheel"),
        ("Your phone", "start ms-settings:mobile-devices"),
    ].iter().cloned().collect();

    let eoa: IndexMap<&str, &str> = [
        ("Audio", "start ms-settings:easeofaccess-audio"),
        ("Closed captions", "start ms-settings:easeofaccess-closedcaptioning"),
        ("Color filters", "start ms-settings:easeofaccess-colorfilter"),
        ("Cursor & pointer size", "start ms-settings:easeofaccess-cursorandpointersize"),
        ("Display", "start ms-settings:easeofaccess-display"),
        ("Eye control", "start ms-settings:easeofaccess-eyecontrol"),
        ("Fonts", "start ms-settings:fonts"),
        ("High contrast", "start ms-settings:easeofaccess-highcontrast"),
        ("Keyboard", "start ms-settings:easeofaccess-keyboard"),
        ("Magnifier", "start ms-settings:easeofaccess-magnifier"),
        ("Mouse", "start ms-settings:easeofaccess-mouse"),
        ("Narrator", "start ms-settings:easeofaccess-narrator"),
        ("Speech", "start ms-settings:easeofaccess-speechrecognition"),
    ].iter().cloned().collect();

    let extras: IndexMap<&str, &str> = [
        ("Extras", "start ms-settings:extras"),
    ].iter().cloned().collect();

    let gaming: IndexMap<&str, &str> = [
        ("Broadcasting", "start ms-settings:gaming-broadcasting"),
        ("Game bar", "start ms-settings:gaming-gamebar"),
        ("Game DVR", "start ms-settings:gaming-gamedvr"),
        ("Game Mode", "start ms-settings:gaming-gamemode"),
        ("Playing a game full screen", "start ms-settings:quietmomentsgame"),
        ("Xbox Networking", "start ms-settings:gaming-xboxnetworking"),
    ].iter().cloned().collect();

    let home: IndexMap<&str, &str> = [
        ("Settings home page", "start ms-settings:"),
    ].iter().cloned().collect();

    let reality: IndexMap<&str, &str> = [
        ("Audio and speech", "start ms-settings:holographic-audio"),
        ("Environment", "start ms-settings:privacy-holographic-environment"),
        ("Headset display", "start ms-settings:holographic-headset"),
        ("Uninstall", "start ms-settings:holographic-management"),
    ].iter().cloned().collect();

    let network: IndexMap<&str, &str> = [
        ("Airplane mode", "start ms-settings:network-airplanemode"),
        ("Cellular and SIM", "start ms-settings:network-cellular"),
        ("Data usage", "start ms-settings:datausage"),
        ("Dial-up", "start ms-settings:network-dialup"),
        ("DirectAccess", "start ms-settings:network-directaccess"),
        ("Ethernet", "start ms-settings:network-ethernet"),
        ("Manage known networks", "start ms-settings:network-wifisettings"),
        ("Mobile hotspot", "start ms-settings:network-mobilehotspot"),
        ("NFC", "start ms-settings:nfctransactions"),
        ("Proxy", "start ms-settings:network-proxy"),
        ("Status", "start ms-settings:network-status"),
        ("VPN", "start ms-settings:network-vpn"),
        ("Wi-Fi", "start ms-settings:network-wifi"),
        ("Wi-Fi Calling", "start ms-settings:network-wificalling"),
    ].iter().cloned().collect();

    let personalization: IndexMap<&str, &str> = [
        ("Background", "start ms-settings:personalization-background"),
        ("Choose which folders appear on Start", "start ms-settings:personalization-start-places"),
        ("Colors", "start ms-settings:personalization-colors"),
        ("Lock screen", "start ms-settings:lockscreen"),
        ("Personalization", "start ms-settings:personalization"),
        ("Start", "start ms-settings:personalization-start"),
        ("Taskbar", "start ms-settings:taskbar"),
        ("Themes", "start ms-settings:themes"),
    ].iter().cloned().collect();

    let phone: IndexMap<&str, &str> = [
        ("Your phone", "start ms-settings:mobile-devices"),
    ].iter().cloned().collect();

    let privacy: IndexMap<&str, &str> = [
        ("Account info", "start ms-settings:privacy-accountinfo"),
        ("Activity history", "start ms-settings:privacy-activityhistory"),
        ("App diagnostics", "start ms-settings:privacy-appdiagnostics"),
        ("Automatic file downloads", "start ms-settings:privacy-automaticfiledownloads"),
        ("Background Apps", "start ms-settings:privacy-backgroundapps"),
        ("Calendar", "start ms-settings:privacy-calendar"),
        ("Call history", "start ms-settings:privacy-callhistory"),
        ("Camera", "start ms-settings:privacy-webcam"),
        ("Contacts", "start ms-settings:privacy-contacts"),
        ("Documents", "start ms-settings:privacy-documents"),
        ("Email", "start ms-settings:privacy-email"),
        ("Eye tracker", "start ms-settings:privacy-eyetracker"),
        ("Feedback and diagnostics", "start ms-settings:privacy-feedback"),
        ("File system", "start ms-settings:privacy-broadfilesystemaccess"),
        ("General", "start ms-settings:privacy-general"),
        ("Location", "start ms-settings:privacy-location"),
        ("Messaging", "start ms-settings:privacy-messaging"),
        ("Microphone", "start ms-settings:privacy-microphone"),
        ("Motion", "start ms-settings:privacy-motion"),
        ("Notifications", "start ms-settings:privacy-notifications"),
        ("Other devices", "start ms-settings:privacy-customdevices"),
        ("Pictures", "start ms-settings:privacy-pictures"),
        ("Radios", "start ms-settings:privacy-radios"),
        ("Speech, inking and typing", "start ms-settings:privacy-speechtyping"),
        ("Tasks", "start ms-settings:privacy-tasks"),
        ("Videos", "start ms-settings:privacy-videos"),
        ("Voice activation", "start ms-settings:privacy-voiceactivation"),
    ].iter().cloned().collect();

    let surface: IndexMap<&str, &str> = [
        ("Accounts", "start ms-settings:surfacehub-accounts"),
        ("Session cleanup", "start ms-settings:surfacehub-sessioncleanup"),
        ("Team Conferencing", "start ms-settings:surfacehub-calling"),
        ("Team device management", "start ms-settings:surfacehub-devicemanagenent"),
        ("Welcome screen", "start ms-settings:surfacehub-welcome"),
    ].iter().cloned().collect();

    let system: IndexMap<&str, &str> = [
        ("About", "start ms-settings:about"),
        ("Advanced display settings", "start ms-settings:display-advanced"),
        ("App volume and device preferences", "start ms-settings:apps-volume"),
        ("Battery Saver", "start ms-settings:batterysaver"),
        ("Battery Saver settings", "start ms-settings:batterysaver-settings"),
        ("Battery use", "start ms-settings:batterysaver-usagedetails"),
        ("Clipboard", "start ms-settings:clipboard"),
        ("Display", "start ms-settings:display"),
        ("Default Save Locations", "start ms-settings:savelocations"),
        ("Display", "start ms-settings:screenrotation"),
        ("Duplicating my display", "start ms-settings:quietmomentspresentation"),
        ("During these hours", "start ms-settings:quietmomentsscheduled"),
        ("Encryption", "start ms-settings:deviceencryption"),
        ("Focus assist", "start ms-settings:quiethours"),
        ("Graphics Settings", "start ms-settings:display-advancedgraphic"),
        ("Messaging", "start ms-settings:messaging"),
        ("Multitasking", "start ms-settings:multitasking"),
        ("Night light settings", "start ms-settings:nightlight"),
        ("Phone", "start ms-settings:phone-defaultapps"),
        ("Projecting to this PC", "start ms-settings:project"),
        ("Shared experiences", "start ms-settings:crossdevice"),
        ("Tablet mode", "start ms-settings:tabletmode"),
        ("Taskbar", "start ms-settings:taskbar"),
        ("Notifications & actions", "start ms-settings:notifications"),
        ("Remote Desktop", "start ms-settings:remotedesktop"),
        ("Power & sleep", "start ms-settings:powersleep"),
        ("Sound", "start ms-settings:sound"),
        ("Storage", "start ms-settings:storagesense"),
        ("Storage Sense", "start ms-settings:storagepolicies"),
    ].iter().cloned().collect();

    let time: IndexMap<&str, &str> = [
        ("Date and time", "start ms-settings:dateandtime"),
        ("Language", "start ms-settings:regionlanguage"),
        ("Speech", "start ms-settings:speech"),
    ].iter().cloned().collect();

    let update: IndexMap<&str, &str> = [
        ("Activation", "start ms-settings:activation"),
        ("Backup", "start ms-settings:backup"),
        ("Delivery Optimization", "start ms-settings:delivery-optimization"),
        ("Find My Device", "start ms-settings:findmydevice"),
        ("For developers", "start ms-settings:developers"),
        ("Recovery", "start ms-settings:recovery"),
        ("Troubleshoot", "start ms-settings:troubleshoot"),
        ("Windows Security", "start ms-settings:windowsdefender"),
        ("Windows Insider Program", "start ms-settings:windowsinsider-optin"),
        ("Windows Update", "start ms-settings:windowsupdate"),
        ("Windows Update: Action", "start ms-settings:windowsupdate-action"),
        ("Windows Update: Advanced options", "start ms-settings:windowsupdate-options"),
        ("Windows Update: Restart options", "start ms-settings:windowsupdate-restartoptions"),
        ("Windows Update: View update history", "start ms-settings:windowsupdate-history"),
    ].iter().cloned().collect();

    let user: IndexMap<&str, &str> = [
        ("Provisioning", "start ms-settings:provisioning"),
        ("Windows Anywhere", "start ms-settings:windowsanywhere"),
        ("Workplace Provisioning", "start ms-settings:workplace-provisioning"),
    ].iter().cloned().collect();

    let menu = Select::with_theme(get_theme())
        .with_prompt("Windows Settings")
        .paged(true)
        .items(&items[..])
        .default(0)
        .interact()
        .expect("Selection could not be made.");

    match menu {
        0 => search(accounts),
        1 => search(apps),
        2 => search(cortana),
        3 => search(devices),
        4 => search(eoa),
        5 => search(extras),
        6 => search(gaming),
        7 => search(home),
        8 => search(reality),
        9 => search(network),
        10 => search(personalization),
        11 => search(phone),
        12 => search(privacy),
        13 => search(surface),
        14 => search(system),
        15 => search(time),
        16 => search(update),
        17 => search(user),
        _ => (),
    };
}

fn search(settings: IndexMap<&str, &str>) {
    Term::stdout()
        .clear_last_lines(1)
        .expect("Line could not be cleared.");

    let mut items = Vec::new();

    for (key, _) in settings.iter() {
        items.push(*key);
    }

    let selections = Checkboxes::with_theme(get_theme())
        .with_prompt("Please select a setting")
        .paged(true)
        .items(&items[..])
        .interact()
        .expect("A selection could not be made.");

    if selections.is_empty() {
        Term::stdout()
            .clear_screen()
            .expect("Line could not be cleared.");

        println!("No setting was selected.");
    } else {
        Term::stdout()
            .clear_screen()
            .expect("Line could not be cleared.");

        for selection in selections {
            if settings.contains_key(items[selection]) {
                match settings.get(items[selection]) {
                    Some(settings) => {
                        update_file(settings);
                    },
                    None => (),
                };
            }
        }
    }

    print!("\n");
    restart();
}
