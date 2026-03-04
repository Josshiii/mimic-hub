fn main() {
    // Iniciamos los atributos por defecto de Tauri
    let mut attrs = tauri_build::Attributes::new();

    #[cfg(target_os = "windows")]
    {
        // Reemplazamos el manifiesto base de Tauri por el nuestro con permisos Admin y soporte HD (DPI)
        let windows = tauri_build::WindowsAttributes::new()
            .app_manifest(r#"
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly manifestVersion="1.0" xmlns="urn:schemas-microsoft-com:asm.v1" xmlns:asmv3="urn:schemas-microsoft-com:asm.v3">
  <dependency>
    <dependentAssembly>
      <assemblyIdentity type="win32" name="Microsoft.Windows.Common-Controls" version="6.0.0.0" processorArchitecture="*" publicKeyToken="6595b64144ccf1df" language="*" />
    </dependentAssembly>
  </dependency>
  <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
      <requestedPrivileges>
        <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
      </requestedPrivileges>
    </security>
  </trustInfo>
  <asmv3:application>
    <asmv3:windowsSettings>
      <dpiAware xmlns="http://schemas.microsoft.com/SMI/2005/WindowsSettings">true/PM</dpiAware>
      <dpiAwareness xmlns="http://schemas.microsoft.com/SMI/2016/WindowsSettings">PerMonitorV2, PerMonitor</dpiAwareness>
    </asmv3:windowsSettings>
  </asmv3:application>
</assembly>
"#);
        
        // Aplicamos la configuración de Windows
        attrs = attrs.windows_attributes(windows);
    }

    // Ejecutamos el constructor con los nuevos atributos
    tauri_build::try_build(attrs).expect("Error al construir Tauri");
}
