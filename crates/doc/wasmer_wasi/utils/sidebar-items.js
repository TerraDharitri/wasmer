window.SIDEBAR_ITEMS = {"constant":[["SNAPSHOT0_NAMESPACE","Namespace for the `Snapshot0` version."],["SNAPSHOT1_NAMESPACE","Namespace for the `Snapshot1` version."],["WASIX_32V1_NAMESPACE","Namespace for the `wasix` version."],["WASIX_64V1_NAMESPACE","Namespace for the `wasix` version."]],"enum":[["WasiVersion","The version of WASI. This is determined by the imports namespace string."]],"fn":[["get_wasi_version","Detect the version of WASI being used based on the import namespaces."],["get_wasi_versions","Like [`get_wasi_version`] but detects multiple WASI versions in a single module. Thus `strict` behaves differently in this function as multiple versions are always supported. `strict` indicates whether non-WASI imports should trigger a failure or be ignored."],["is_wasi_module","Check if a provided module is compiled for some version of WASI. Use [`get_wasi_version`] to find out which version of WASI the module is."],["is_wasix_module","Returns if the module is WASIX or not"],["map_io_err",""]]};