use windows::Win32::Foundation::{HANDLE, LUID};
use windows::Win32::Security::*;
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

fn main() {
    let tokens = vec![
        SE_ASSIGNPRIMARYTOKEN_NAME,
        SE_AUDIT_NAME,
        SE_BACKUP_NAME,
        SE_CHANGE_NOTIFY_NAME,
        SE_CREATE_GLOBAL_NAME,
        SE_CREATE_PAGEFILE_NAME,
        SE_CREATE_PERMANENT_NAME,
        SE_CREATE_SYMBOLIC_LINK_NAME,
        SE_CREATE_TOKEN_NAME,
        SE_DEBUG_NAME,
        SE_DELEGATE_SESSION_USER_IMPERSONATE_NAME,
        SE_ENABLE_DELEGATION_NAME,
        SE_IMPERSONATE_NAME,
        SE_INCREASE_QUOTA_NAME,
        SE_INC_BASE_PRIORITY_NAME,
        SE_INC_WORKING_SET_NAME,
        SE_LOAD_DRIVER_NAME,
        SE_LOCK_MEMORY_NAME,
        SE_MACHINE_ACCOUNT_NAME,
        SE_MANAGE_VOLUME_NAME,
        SE_PROF_SINGLE_PROCESS_NAME,
        SE_RELABEL_NAME,
        SE_REMOTE_SHUTDOWN_NAME,
        SE_RESTORE_NAME,
        SE_SECURITY_NAME,
        SE_SHUTDOWN_NAME,
        SE_SYNC_AGENT_NAME,
        SE_SYSTEMTIME_NAME,
        SE_SYSTEM_ENVIRONMENT_NAME,
        SE_SYSTEM_PROFILE_NAME,
        SE_TAKE_OWNERSHIP_NAME,
        SE_TCB_NAME,
        SE_TIME_ZONE_NAME,
        SE_TRUSTED_CREDMAN_ACCESS_NAME,
        SE_UNDOCK_NAME,
        SE_UNSOLICITED_INPUT_NAME,
    ];

    unsafe {
        let mut h_token = HANDLE::default();
        let mut token_privileges = TOKEN_PRIVILEGES {
            PrivilegeCount: 1,
            Privileges: [LUID_AND_ATTRIBUTES {
                Luid: LUID::default(),
                Attributes: SE_PRIVILEGE_ENABLED,
            }; 1],
        };

        let _ = OpenProcessToken(
            GetCurrentProcess(),
            TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY,
            &mut h_token,
        );
        for token in tokens {
            let _ = LookupPrivilegeValueW(
                None,
                token,
                &mut token_privileges.Privileges[0].Luid as *mut LUID,
            );
    
            let _ = AdjustTokenPrivileges(h_token, false, Some(&token_privileges), 0, None, None);
        }
    }
}
