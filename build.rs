


fn main() {

   println!("cargo:rerun-if-changed=target/debug/assetchunk.dll");
   if !std::path::Path::new("target/debug/assetchunk.dll").exists() {
      eprintln!("[-] DLL Not Found In Target Directory");
      eprintln!("[-] Build Failed");
      return;
   }
   if !std::path::Path::new("ctest").exists() {
      std::fs::create_dir("ctest")
         .expect("[-] Failed To Create CTest Directory");
   }
   eprintln!("[-] Checking For Old DLL In CTest Directory");
   if std::path::Path::new("ctest/assetchunk.dll").exists() {
      std::fs::remove_file("ctest/assetchunk.dll")
         .expect("[-] Failed To Remove Old DLL From CTest Directory");
   }
   eprintln!("[+] Copying DLL To CTest Directory");
   std::fs::copy("target/debug/assetchunk.dll", "ctest/assetchunk.dll")
      .expect("[-] Failed To Copy DLL To CTest Directory");
   eprintln!("[+] Build Complete");
}
