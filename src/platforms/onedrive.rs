use crate::constant_arrays;
use crate::Platform;

pub struct OneDrive {}

impl OneDrive {
    constant_arrays! {
        /// https://support.microsoft.com/en-us/office/restrictions-and-limitations-in-onedrive-and-sharepoint-64883a5d-228e-48f5-b3d2-eb39e07630fa
        const FORBIDDEN_CHARACTERS_FOLDER: [char; _] = ['"', '*', ':', '<', '>', '?', '/', '\\', '|'];
        /// https://support.microsoft.com/en-us/office/restrictions-and-limitations-in-onedrive-and-sharepoint-64883a5d-228e-48f5-b3d2-eb39e07630fa
        const FORBIDDEN_CHARACTERS_FILE: [char; _] = [
            '~', '"', '#', '%', '&', '*', ':', '<', '>', '?', '/', '\\', '{', '|', '}',
        ];
    }
}
