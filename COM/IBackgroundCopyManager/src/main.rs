use windows::{
    core::{GUID, HSTRING},
    Win32::{
        Networking::BackgroundIntelligentTransferService::{
            IBackgroundCopyManager, BG_JOB_STATE, 
            BG_JOB_STATE_ERROR, BG_JOB_STATE_TRANSFERRED, 
            BG_JOB_TYPE_DOWNLOAD, BG_JOB_TYPE_UPLOAD,
        },
        System::Com::{
            CoCreateInstance, CoInitializeEx, 
            CoUninitialize, CLSCTX_ALL, COINIT_MULTITHREADED,
        },
    },
};

/// CLSID for the Background Intelligent Transfer Service manager
pub const CLSID_IBACKGROUND_COPY_MANAGER: GUID = GUID::from_u128(0x4991d34b_80a1_4291_83b6_3328366b9097);

fn main() -> windows::core::Result<()> {
    unsafe {
        // Initialize COM in multithreaded mode
        CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;

        // Start the BITS download and upload flows
        download("", "")?;
        upload()?;

        // Cleanup COM after operations complete
        CoUninitialize();
    }

    Ok(())
}

/// Creates and runs a BITS download job using the BackgroundCopyManager.
///
/// This function creates a new job, adds a file from an HTTP source,
/// resumes the job, and polls for completion or error status.
fn download(url: &str, save: &str) -> windows::core::Result<()> {
    unsafe {
        // Create instance of BITS manager
        let back: IBackgroundCopyManager = CoCreateInstance(&CLSID_IBACKGROUND_COPY_MANAGER, None, CLSCTX_ALL)?;

        // Prepare job GUID and holder
        let mut guid = GUID::zeroed();
        let mut job = None;

        // Create a download job
        back.CreateJob(&HSTRING::from("rust"), BG_JOB_TYPE_DOWNLOAD, &mut guid, &mut job)?;

        let cpjob = job.expect("Failed to create download job");

        // Add file to download: source URL and destination path
        cpjob.AddFile(
            &HSTRING::from(url),
            &HSTRING::from(save),
        )?;

        // Resume the job
        if cpjob.Resume().is_ok() {
            // Monitor state until transfer finishes or fails
            let mut state = BG_JOB_STATE::default();
            while state != BG_JOB_STATE_ERROR && state != BG_JOB_STATE_TRANSFERRED {
                state = cpjob.GetState()?;
                std::thread::sleep(std::time::Duration::from_millis(300));
                print!(".")
            }

            // Handle final state
            if state == BG_JOB_STATE_ERROR {
                println!("[!] Error in transfer");
            } else {
                cpjob.Complete()?;
                println!("[+] Transfer Successful!");
            }
        }
    }

    Ok(())
}

/// Creates and runs a BITS upload job using the BackgroundCopyManager.
///
/// This function creates a new job, adds a local file to upload to an HTTP endpoint,
/// resumes the job, and polls until it's complete or fails.
fn upload() -> windows::core::Result<()> {
    unsafe {
        // Create instance of BITS manager
        let back: IBackgroundCopyManager = CoCreateInstance(&CLSID_IBACKGROUND_COPY_MANAGER, None, CLSCTX_ALL)?;

        // Prepare job GUID and holder
        let mut guid = GUID::zeroed();
        let mut job = None;

        // Create an upload job
        back.CreateJob(&HSTRING::from("rust-upload"), BG_JOB_TYPE_UPLOAD, &mut guid, &mut job)?;

        let cpjob = job.expect("Failed to create upload job");

        // Add file to upload: destination endpoint and local source path
        cpjob.AddFile(
            &HSTRING::from("http://127.0.0.1:8081/file.bin"),
            &HSTRING::from("C:\\Windows\\Temp\\shell.bin"),
        )?;

        // Resume the job
        if cpjob.Resume().is_ok() {
            println!("Uploading .....");

            // Monitor state until transfer finishes or fails
            let mut state = BG_JOB_STATE::default();
            while state != BG_JOB_STATE_ERROR && state != BG_JOB_STATE_TRANSFERRED {
                state = cpjob.GetState()?;
                std::thread::sleep(std::time::Duration::from_millis(300));
                print!(".")
            }

            // Handle final state
            if state == BG_JOB_STATE_ERROR {
                println!("[!] Error in transfer");
            } else {
                cpjob.Complete()?;
                println!("[+] Transfer Successful!");
            }
        }
    }

    Ok(())
}
