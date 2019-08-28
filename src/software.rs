use console::Term;
use dialoguer::{
    Checkboxes,
    Confirmation,
    Select
};
use indexmap::IndexMap;

use crate::main;
use crate::utils::{
    get_theme,
    get_windows_path,
    path_exists,
    update_file
};


fn restart() {
    if Confirmation::with_theme(get_theme())
        .with_text("Do you want to search for more software?")
        .interact()
        .expect("Confirmation could not be accepted.") {
            microsoft_software_menu();
    } else {
        Term::stdout()
            .clear_screen()
            .expect("Terminal could not be cleared.");

        main();
    }
}

pub fn microsoft_software_menu() {
    Term::stdout()
        .clear_screen()
        .expect("Line could not be cleared.");

    // Microsoft Software Menu
    let items = &[
        "Apps",
        "Basic",
        "Advanced",
    ];

    let apps: IndexMap<&str, &str> = [
        ("3D Builder", "start com.microsoft.builder3d:"),
        ("Action Center", "start ms-actioncenter:"),
        ("Alarms and Clock", "start ms-clock:"),
        ("Available Networks", "start ms-availablenetworks:"),
        ("Calculator", "start calculator:"),
        ("Calendar", "start outlookcal:"),
        ("Camera", "start microsoft.windows.camera:"),
        ("Candy Crush Soda Saga", "start candycrushsodasaga:"),
        ("Connect", "start ms-projection:"),
        ("Cortana", "start ms-cortana:"),
        ("Cortana Connected Services", "start ms-cortana://notebook/?ConnectedServices"),
        ("Cortana Personal Information", "start ms-cortana://settings/ManageBingProfile"),
        ("Device Discovery", "start ms-settings-connectabledevices:devicediscovery"),
        ("Drawboard PDF", "start drawboardpdf:"),
        ("Facebook", "start fb:"),
        ("Feedback Hub", "start feedback-hub:"),
        ("Get Help", "start ms-contact-support:"),
        ("Groove Music", "start mswindowsmusic:"),
        ("Mail", "start outlookmail:"),
        ("Maps", "start bingmaps:"),
        ("Messaging", "start ms-chat:"),
        ("Microsoft Edge", "start microsoft-edge:"),
        ("Microsoft News", "start bingnews:"),
        ("Microsoft Solitaire Collection", "start xboxliveapp-1297287741:"),
        ("Microsoft Store", "start ms-windows-store:"),
        ("Microsoft Store: Music", "start microsoftmusic:"),
        ("Microsoft Store: Movies and TV", "start microsoftvideo:"),
        ("Microsoft Whiteboard", "start ms-whiteboard-cmd:"),
        ("Minecraft: Windows 10 Edition", "start minecraft:"),
        ("Mixed Reality Camera", "start ms-holocamera:"),
        ("Mixed Reality Portal", "start ms-holographicfirstrun:"),
        ("Movies and TV", "start mswindowsvideo:"),
        ("OneNote", "start onenote:"),
        ("Paint 3D", "start ms-paint:"),
        ("People", "start ms-people:"),
        ("Photos", "start ms-photos:"),
        ("Project Display", "start ms-settings-displays-topology:projection"),
        ("Settings", "start ms-settings:"),
        ("Tips", "start ms-get-started:"),
        ("Twitter", "start twitter:"),
        ("View 3D Preview", "start com.microsoft.3dviewer:"),
        ("Voice Recorder", "start ms-callrecording:"),
        ("Weather", "start bingweather:"),
        ("Windows Mixed Reality Environments", "start ms-environment-builder:"),
        ("Windows Parental Controls", "start ms-wpc:"),
        ("Windows Security", "start windowsdefender:"),
        ("Xbox", "start xbox:"),
        ("Xbox - Friends list", "start xbox-friendfinder:"),
        ("Xbox - Profile page", "start xbox-profile:"),
        ("Xbox - Network settings", "start xbox-network:"),
        ("Xbox - Settings", "start xbox-settings:"),
        ("Xbox One SmartGlass", "start smartglass:"),
    ].iter().cloned().collect();

    let basic: IndexMap<&str, &str> = [
        ("Calculator", "\\system32\\calc.exe"),
        ("Command Prompt", "\\system32\\cmd.exe"),
        ("Control Panel", "\\system32\\control.exe"),
        ("Disk Cleanup", "\\system32\\cleanmgr.exe"),
        ("Disk Defragmenter", "\\system32\\dfrgui.exe"),
        ("Magnify", "\\system32\\magnify.exe"),
        ("Malicious Software Removal Tool", "\\system32\\mrt.exe"),
        ("Microsoft Windows Backup", "\\system32\\sdclt.exe"),
        ("Narrator", "\\system32\\narrator.exe"),
        ("On-Screen Keyboard", "\\system32\\osk.exe"),
        ("Paint", "\\system32\\mspaint.exe"),
        ("Performance Monitor", "\\system32\\perfmon.exe"),
        ("Registry Editor", "\\system32\\regedt32.exe"),
        ("Remote Desktop Connection", "\\system32\\mstsc.exe"),
        ("Resource Monitor", "\\system32\\resmon.exe"),
        ("Set Program Access and Computer Defaults", "\\system32\\computerdefaults.exe"),
        ("System Configuration", "\\system32\\msconfig.exe"),
        ("System Information", "\\system32\\msinfo32.exe"),
        ("System Restore", "\\system32\\rstrui.exe"),
        ("Task Manager", "\\system32\\taskmgr.exe"),
        ("Volume Mixer", "\\system32\\sndvol.exe"),
        ("Windows Explorer", "\\explorer.exe"),
        ("Windows Memory Diagnostics Tool", "\\system32\\mdsched.exe"),
        ("Wordpad", "\\system32\\write.exe"),
    ].iter().cloned().collect();

    let advanced: IndexMap<&str, &str> = [
        ("Advanced User Accounts", "\\system32\\Netplwiz.exe"),
        ("Bluetooth File Transfer", "\\system32\\fsquirt.exe"),
        ("Character Map", "\\system32\\charmap.exe"),
        ("ClearType Tuner", "\\system32\\cttune.exe"),
        ("Computer Management", "\\system32\\compmgmtlauncher.exe"),
        ("Credential Backup and Restore Wizard", "\\system32\\credwiz.exe"),
        ("Device Pairing Wizard", "\\system32\\DevicePairingWizard.exe"),
        ("Direct X Control Panel", "\\system32\\dxcpl.exe"),
        ("Display Color Calibration", "\\system32\\dccw.exe"),
        ("Display Switcher", "\\system32\\DisplaySwitch.exe"),
        ("DPI Scaling", "\\system32\\DpiScaling.exe"),
        ("Driver Verifier Manager", "\\system32\\verifiergui.exe"),
        ("Event Viewer", "\\system32\\eventvwr.exe"),
        ("File History", "\\system32\\filehistory.exe"),
        ("iExpress", "\\system32\\iexpress.exe"),
        ("Microsoft Color Control Panel", "\\system32\\colorcpl.exe"),
        ("Microsoft iSCSI Initiator Configuration Tool", "\\system32\\iscsicpl.exe"),
        ("Microsoft Management Console", "\\system32\\mmc.exe"),
        ("Microsoft Support Diagnostics Tool", "\\system32\\msdt.exe"),
        ("Microsoft Sync Center", "\\system32\\mobsync.exe"),
        ("ODBC Administrator", "\\system32\\odbcad32.exe"),
        ("Phone Dialer", "\\system32\\dialer.exe"),
        ("Powershell", "\\system32\\windowspowershell\\v1.0\\powershell.exe"),
        ("Powershell ISE", "\\system32\\windowspowershell\\v1.0\\powershell_ise.exe"),
        ("Private Character Editor", "\\system32\\eudcedit.exe"),
        ("Recovery Drive", "\\system32\\recoverydrive.exe"),
        ("SQL Client Configuration", "\\system32\\cliconfg.exe"),
        ("Steps Recorder", "\\system32\\psr.exe"),
        ("System Preparation Tool", "\\system32\\sysprep\\sysprep.exe"),
        ("System Properties: Advanced", "\\system32\\systempropertiesadvanced.exe"),
        ("System Properties: Computer Name", "\\system32\\systempropertiescomputerName.exe"),
        ("System Properties: Data Execution Prevention", "\\system32\\systempropertiesdataexecutionprevention.exe"),
        ("System Properties: Hardware", "\\system32\\systempropertieshardware.exe"),
        ("System Properties: Performance", "\\system32\\systempropertiesperformance.exe"),
        ("System Properties: Protection", "\\system32\\systempropertiesprotection.exe"),
        ("System Properties: Remote", "\\system32\\systempropertiesremote.exe"),
        ("User Account Control Settings", "\\system32\\useraccountcontrolsettings.exe"),
        ("Utility Manager", "\\system32\\utilman.exe"),
        ("Windows Activation", "\\system32\\slui.exe"),
        ("Windows Features", "\\system32\\optionalfeatures.exe"),
        ("Windows Fax and Scan", "\\system32\\wfs.exe"),
        ("Windows Mobility Center", "\\system32\\mblctr.exe"),
        ("Windows Picture Acquisition Wizard", "\\system32\\wiaacmgr.exe"),
        ("Windows Remote Assistance", "\\system32\\msra.exe"),
        ("WMI Command Line Utility", "\\system32\\wbem\\wmic.exe"),
        ("WMI Test Tool", "\\system32\\wbem\\wbemtest.exe"),
        ("Work Folders", "\\system32\\workfolders.exe"),
    ].iter().cloned().collect();

    let menu = Select::with_theme(get_theme())
        .with_prompt("Windows Software")
        .paged(true)
        .items(&items[..])
        .default(0)
        .interact()
        .expect("Selection could not be made.");

    match menu {
        0 => search(apps),
        1 => search(basic),
        2 => search(advanced),
        _ => (),
    };
}

fn search(software: IndexMap<&str, &str>) {
    Term::stdout()
        .clear_last_lines(1)
        .expect("Line could not be cleared.");

    let mut items = Vec::new();

    for (key, _) in software.iter() {
        items.push(*key);
    }

    let selections = Checkboxes::with_theme(get_theme())
        .with_prompt("Please select a program")
        .paged(true)
        .items(&items[..])
        .interact()
        .expect("Selection could not be made.");

    if selections.is_empty() {
        Term::stdout()
            .clear_screen()
            .expect("Line could not be cleared.");

        println!("No program was selected.");
    } else {
        Term::stdout()
            .clear_screen()
            .expect("Line could not be cleared.");

        let windows = get_windows_path();

        for selection in selections {
            if software.contains_key(items[selection]) {
                match software.get(items[selection]) {
                    Some(software) => {
                        let path = format!("{}{}", windows.to_string_lossy(), software);

                        if path_exists(&path) {
                            update_file(&path);
                        } else {
                            update_file(software);
                        }
                    },
                    None => (),
                };
            }
        }
    }

    print!("\n");
    restart();
}
