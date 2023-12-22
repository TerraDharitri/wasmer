(function() {var implementors = {
"virtual_fs":[],
"wasmer_wasix":[["impl <a class=\"trait\" href=\"wasmer_wasix/syscalls/trait.FileSystem.html\" title=\"trait wasmer_wasix::syscalls::FileSystem\">FileSystem</a> for <a class=\"enum\" href=\"wasmer_wasix/fs/enum.WasiFsRoot.html\" title=\"enum wasmer_wasix::fs::WasiFsRoot\">WasiFsRoot</a>"],["impl <a class=\"trait\" href=\"wasmer_wasix/syscalls/trait.FileSystem.html\" title=\"trait wasmer_wasix::syscalls::FileSystem\">FileSystem</a> for <a class=\"struct\" href=\"wasmer_wasix/fs/struct.FallbackFileSystem.html\" title=\"struct wasmer_wasix::fs::FallbackFileSystem\">FallbackFileSystem</a>"],["impl&lt;M, F&gt; <a class=\"trait\" href=\"wasmer_wasix/syscalls/trait.FileSystem.html\" title=\"trait wasmer_wasix::syscalls::FileSystem\">FileSystem</a> for <a class=\"struct\" href=\"wasmer_wasix/runtime/package_loader/load_package_tree/struct.MappedPathFileSystem.html\" title=\"struct wasmer_wasix::runtime::package_loader::load_package_tree::MappedPathFileSystem\">MappedPathFileSystem</a>&lt;F, M&gt;<span class=\"where fmt-newline\">where\n    F: <a class=\"trait\" href=\"wasmer_wasix/syscalls/trait.FileSystem.html\" title=\"trait wasmer_wasix::syscalls::FileSystem\">FileSystem</a>,\n    M: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/ops/function/trait.Fn.html\" title=\"trait core::ops::function::Fn\">Fn</a>(&amp;<a class=\"struct\" href=\"wasmer_wasix/syscalls/struct.Path.html\" title=\"struct wasmer_wasix::syscalls::Path\">Path</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.70.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.70.0/std/path/struct.PathBuf.html\" title=\"struct std::path::PathBuf\">PathBuf</a>, <a class=\"enum\" href=\"wasmer_wasix/enum.FsError.html\" title=\"enum wasmer_wasix::FsError\">FsError</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,</span>"],["impl&lt;F: <a class=\"trait\" href=\"wasmer_wasix/syscalls/trait.FileSystem.html\" title=\"trait wasmer_wasix::syscalls::FileSystem\">FileSystem</a>&gt; <a class=\"trait\" href=\"wasmer_wasix/syscalls/trait.FileSystem.html\" title=\"trait wasmer_wasix::syscalls::FileSystem\">FileSystem</a> for <a class=\"struct\" href=\"wasmer_wasix/runners/wasi_common/struct.RelativeOrAbsolutePathHack.html\" title=\"struct wasmer_wasix::runners::wasi_common::RelativeOrAbsolutePathHack\">RelativeOrAbsolutePathHack</a>&lt;F&gt;"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()