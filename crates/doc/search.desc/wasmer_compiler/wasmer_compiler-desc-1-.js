searchState.loadedDescShard("wasmer_compiler", 1, "Multiple values.\nA single value.\nA helper enum for representing either a single or multiple …\nMap of signatures to a function’s parameter and return …\nGet the parameter and result types for the given Wasm …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nTrue if empty.\nIterate ofer the value types.\nCount of values.\nCreates a new empty ModuleTranslationState.\nA map containing a Wasm module’s original, raw …\nData structures to provide transformation of the source\nA <code>Compilation</code> contains the compiled function bodies for a …\nTypes for modules.\nRelocation is the process of assigning load addresses for …\nThis module define the required structures to emit custom …\nThis module define the required structures for compilation …\nTarget configuration\nA <code>CompiledFunctionUnwindInfo</code> contains the function unwind …\nAn archived <code>FunctionAddressMap</code>\nAn archived <code>InstructionAddressMap</code>\nFunction and its instructions addresses mappings.\nThe resolver for an archived <code>FunctionAddressMap</code>\nSingle source location to generated address mapping.\nThe resolver for an archived <code>InstructionAddressMap</code>\nGenerated function body length.\nGenerated function body offset if applicable, otherwise 0.\nGenerated instructions length.\nGenerated instructions offset.\nFunction end source location.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nInstructions maps. The array is sorted by the …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nOriginal source location.\nFunction start source location (normally declaration).\nAn archived <code>CompiledFunction</code>\nAn archived <code>CompiledFunctionFrameInfo</code>\nAn archived <code>Dwarf</code>\nAn archived <code>FunctionBody</code>\nThe result of compiling a WebAssembly module’s functions.\nThe result of compiling a WebAssembly function.\nThe frame info for a Compiled function.\nThe resolver for an archived <code>CompiledFunctionFrameInfo</code>\nThe resolver for an archived <code>CompiledFunction</code>\nThe custom sections for a Compilation.\nThe DWARF information for this Compilation.\nThe resolver for an archived <code>Dwarf</code>\nThe function body.\nAny struct that acts like a <code>FunctionBody</code>.\nThe resolver for an archived <code>FunctionBody</code>\nThe compiled functions map (index in the Wasm -&gt; function)\nThe address map.\nThe function body bytes.\nThe function body.\nCustom sections for the module. It will hold the data, for …\nSection ids corresponding to the Dwarf debug info\nTrampolines to call a dynamic function defined in a host, …\nThe section index in the <code>Compilation</code> that corresponds to …\nThe frame information.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nTrampolines to call a function defined locally in the wasm …\nCompiled code for the function bodies.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCreates a <code>Dwarf</code> struct with the corresponding indices for …\nThe relocations (in the body)\nThe traps (in the function body).\nThe function unwind info\nAn archived <code>CompileModuleInfo</code>\nThe required info for compiling a module.\nThe resolver for an archived <code>CompileModuleInfo</code>\nThe features used for compiling the module\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThe memory styles used for compiling.\nThe module information\nThe table plans used for compiling.\nThe resolver for <code>RelocationKind::Aarch64AddAbsLo12Nc</code>\nThe archived counterpart of …\nR_AARCH64_ADD_ABS_LO12_NC\nThe resolver for <code>RelocationKind::Aarch64AdrPrelLo21</code>\nThe archived counterpart of …\nR_AARCH64_ADR_PREL_LO21\nThe resolver for <code>RelocationKind::Aarch64AdrPrelPgHi21</code>\nThe archived counterpart of …\nR_AARCH64_ADR_PREL_PG_HI21\nThe resolver for <code>RelocationKind::Aarch64Ldst128AbsLo12Nc</code>\nThe archived counterpart of …\nR_AARCH64_LDST128_ABS_LO12_NC\nThe resolver for <code>RelocationKind::Aarch64Ldst64AbsLo12Nc</code>\nThe archived counterpart of …\nR_AARCH64_LDST64_ABS_LO12_NC\nThe resolver for <code>RelocationKind::Abs4</code>\nThe archived counterpart of <code>RelocationKind::Abs4</code>\nabsolute 4-byte\nThe resolver for <code>RelocationKind::Abs8</code>\nThe archived counterpart of <code>RelocationKind::Abs8</code>\nabsolute 8-byte\nAn archived <code>Relocation</code>\nAn archived <code>RelocationKind</code>\nAn archived <code>RelocationTarget</code>\nThe resolver for <code>RelocationKind::Arm32Call</code>\nThe archived counterpart of <code>RelocationKind::Arm32Call</code>\nArm32 call target\nThe resolver for <code>RelocationKind::Arm64Call</code>\nThe archived counterpart of <code>RelocationKind::Arm64Call</code>\nArm64 call target\nThe resolver for <code>RelocationKind::Arm64Movw0</code>\nThe archived counterpart of <code>RelocationKind::Arm64Movw0</code>\nArm64 movk/z part 0\nThe resolver for <code>RelocationKind::Arm64Movw1</code>\nThe archived counterpart of <code>RelocationKind::Arm64Movw1</code>\nArm64 movk/z part 1\nThe resolver for <code>RelocationKind::Arm64Movw2</code>\nThe archived counterpart of <code>RelocationKind::Arm64Movw2</code>\nArm64 movk/z part 2\nThe resolver for <code>RelocationKind::Arm64Movw3</code>\nThe archived counterpart of <code>RelocationKind::Arm64Movw3</code>\nArm64 movk/z part 3\nThe resolver for <code>RelocationTarget::CustomSection</code>\nThe archived counterpart of <code>RelocationTarget::CustomSection</code>\nCustom sections generated by the compiler\nThe resolver for <code>RelocationKind::ElfX86_64TlsGd</code>\nThe archived counterpart of <code>RelocationKind::ElfX86_64TlsGd</code>\nElf x86_64 32 bit signed PC relative offset to two GOT …\nThe resolver for <code>RelocationKind::LArchAbs64Hi12</code>\nThe archived counterpart of <code>RelocationKind::LArchAbs64Hi12</code>\nLoongArch absolute high 12bit\nThe resolver for <code>RelocationKind::LArchAbs64Lo20</code>\nThe archived counterpart of <code>RelocationKind::LArchAbs64Lo20</code>\nLoongArch absolute low 20bit\nThe resolver for <code>RelocationKind::LArchAbsHi20</code>\nThe archived counterpart of <code>RelocationKind::LArchAbsHi20</code>\nLoongArch absolute high 20bit\nThe resolver for <code>RelocationKind::LArchAbsLo12</code>\nThe archived counterpart of <code>RelocationKind::LArchAbsLo12</code>\nLoongArch absolute low 12bit\nThe resolver for <code>RelocationKind::LArchCall36</code>\nThe archived counterpart of <code>RelocationKind::LArchCall36</code>\nLoongArch PC-relative call 38bit\nThe resolver for <code>RelocationKind::LArchPCAla64Hi12</code>\nThe archived counterpart of …\nLoongArch PC64-relative high 12bit\nThe resolver for <code>RelocationKind::LArchPCAla64Lo20</code>\nThe archived counterpart of …\nLoongArch PC64-relative low 20bit\nThe resolver for <code>RelocationKind::LArchPCAlaHi20</code>\nThe archived counterpart of <code>RelocationKind::LArchPCAlaHi20</code>\nLoongArch PC-relative high 20bit\nThe resolver for <code>RelocationKind::LArchPCAlaLo12</code>\nThe archived counterpart of <code>RelocationKind::LArchPCAlaLo12</code>\nLoongArch PC-relative low 12bit\nThe resolver for <code>RelocationTarget::LibCall</code>\nThe archived counterpart of <code>RelocationTarget::LibCall</code>\nA compiler-generated libcall.\nThe resolver for <code>RelocationTarget::LocalFunc</code>\nThe archived counterpart of <code>RelocationTarget::LocalFunc</code>\nA relocation to a function defined locally in the wasm …\nA record of a relocation to perform.\nRelocation kinds for every ISA.\nThe resolver for an archived <code>RelocationKind</code>\nAny struct that acts like a <code>Relocation</code>.\nThe resolver for an archived <code>Relocation</code>\nDestination function. Can be either user function or some …\nThe resolver for an archived <code>RelocationTarget</code>\nRelocations to apply to function bodies.\nThe resolver for <code>RelocationKind::RiscvCall</code>\nThe archived counterpart of <code>RelocationKind::RiscvCall</code>\nRISC-V call target\nThe resolver for <code>RelocationKind::RiscvPCRelHi20</code>\nThe archived counterpart of <code>RelocationKind::RiscvPCRelHi20</code>\nRISC-V PC-relative high 20bit\nThe resolver for <code>RelocationKind::RiscvPCRelLo12I</code>\nThe archived counterpart of <code>RelocationKind::RiscvPCRelLo12I</code>\nRISC-V PC-relative low 12bit, I-type\nThe resolver for <code>RelocationKind::X86CallPCRel4</code>\nThe archived counterpart of <code>RelocationKind::X86CallPCRel4</code>\nx86 call to PC-relative 4-byte\nThe resolver for <code>RelocationKind::X86CallPLTRel4</code>\nThe archived counterpart of <code>RelocationKind::X86CallPLTRel4</code>\nx86 call to PLT-relative 4-byte\nThe resolver for <code>RelocationKind::X86GOTPCRel4</code>\nThe archived counterpart of <code>RelocationKind::X86GOTPCRel4</code>\nx86 GOT PC-relative 4-byte\nThe resolver for <code>RelocationKind::X86PCRel4</code>\nThe archived counterpart of <code>RelocationKind::X86PCRel4</code>\nx86 PC-relative 4-byte\nThe resolver for <code>RelocationKind::X86PCRel8</code>\nThe archived counterpart of <code>RelocationKind::X86PCRel8</code>\nx86 PC-relative 8-byte\nThe addend to add to the relocation value.\nDisplay trait implementation drops the arch, since its …\nGiven a function start address, provide the relocation …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThe relocation kind.\nThe offset where to apply the relocation.\nRelocation target.\nAn archived <code>CustomSection</code>\nAn archived <code>CustomSectionProtection</code>\nAn archived <code>SectionBody</code>\nAn archived <code>SectionIndex</code>\nA Section for a <code>Compilation</code>.\nAny struct that acts like a <code>CustomSection</code>.\nCustom section Protection.\nThe resolver for an archived <code>CustomSectionProtection</code>\nThe resolver for an archived <code>CustomSection</code>\nThe resolver for <code>CustomSectionProtection::Read</code>\nThe archived counterpart of <code>CustomSectionProtection::Read</code>\nA custom section with read permission.\nThe resolver for <code>CustomSectionProtection::ReadExecute</code>\nThe archived counterpart of …\nA custom section with read and execute permissions.\nThe bytes in the section.\nThe resolver for an archived <code>SectionBody</code>\nIndex type of a Section defined inside a WebAssembly …\nThe resolver for an archived <code>SectionIndex</code>\nReturns a raw pointer to the section’s buffer.\nDereferences into the section’s buffer.\nReturn the underlying index value as a <code>u32</code>.\nThe bytes corresponding to this section.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCreate a new instance from a <code>u32</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns whether or not the section body is empty.\nReturns whether or not the section body is empty.\nReturns the length of this section in bytes.\nReturns the length of this section in bytes.\nCreate a new section body with the given contents.\nMemory protection that applies to this section.\nRelocations that apply to this custom section.\nAn archived <code>ModuleMetadata</code>\nAn archived <code>Symbol</code>\nThe resolver for <code>Symbol::DynamicFunctionTrampoline</code>\nThe archived counterpart of …\nThe dynamic function trampoline for a given function.\nThe resolver for <code>Symbol::FunctionCallTrampoline</code>\nThe archived counterpart of <code>Symbol::FunctionCallTrampoline</code>\nThe function call trampoline for a given signature.\nThe resolver for <code>Symbol::LocalFunction</code>\nThe archived counterpart of <code>Symbol::LocalFunction</code>\nA function defined in the wasm.\nThe resolver for <code>Symbol::Metadata</code>\nThe archived counterpart of <code>Symbol::Metadata</code>\nA metadata section, indexed by a unique prefix (usually …\nSerializable struct that represents the compiled metadata.\nThe resolver for an archived <code>ModuleMetadata</code>\nA simple metadata registry\nThe resolver for <code>Symbol::Section</code>\nThe archived counterpart of <code>Symbol::Section</code>\nA wasm section.\nThe kinds of wasmer_types objects that might be found in a …\nThis trait facilitates symbol name lookups in a native …\nThe resolver for an archived <code>Symbol</code>\nSafety\nSafety\nCompile info\nCPU features used (See <code>CpuFeature</code>)\nData initializers\nDeserialize a Module from a slice. The slice must have the …\nDeserialize a compilation module from an archive\nDeserialize a Module from a slice. The slice must have the …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nThe function body lengths (used to find function by …\nReturns symbol registry.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nGiven a name it returns the <code>Symbol</code> for that name in the …\nSymbol prefix stirng\nPrefix for function etc symbols\nSerialize a Module into bytes The bytes will have the …\nGet mutable ref to compile info and a copy of the registry\nGiven a <code>Symbol</code> it returns the name for that symbol in the …\nApple Aarch64 platforms use their own variant of the …\nThe “architecture” field, which in some cases also …\nThe “binary format” field, which is usually omitted, …\nThe calling convention, which specifies things like which …\nThe nomenclature is inspired by the <code>cpuid</code> crate. The list …\nA custom vendor. “Custom” in this context means that …\nThe target memory endianness.\nThe “environment” field, which specifies an ABI …\nThe “operating system” field, which sometimes implies …\nThe width of a pointer (in the default address space).\n“System V”, which is used on most Unix-like platfoms. …\nThis is the target that we will use for compiling the …\nA target “triple”. Historically such things had three …\nThe “vendor” field, which in practice is little more …\nThe WebAssembly C ABI. …\n“Windows Fastcall”, which is used on Windows. Note …\nx86_64 target that only supports Haswell-compatible Intel …\nThe “architecture” (and sometimes the subarchitecture).\nExtracts a string slice.\nThe “binary format” (rarely used).\nReturn the number of bits in a pointer.\nReturn the number of bytes in a pointer.\nThe triple associated for the target.\nThe C data model for a given target. If the model is not …\nReturn the default calling convention for the given target …\nReturn the endianness of this architecture.\nReturn the endianness of this architecture.\nReturn the endianness of this target’s architecture.\nThe “environment” on top of the operating system …\nRetrieves the features for the current Host\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturn the architecture for the current host.\nReturn the vendor for the current host.\nReturn the operating system for the current host.\nReturn the environment for the current host.\nReturn the binary format for the current host.\nReturn the triple for the current host.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConvert into a string\nConvert into a string\nConvert into a string\nConvert into a string\nConvert into a string\nChecks if this Architecture is some variant of Clever-ISA\nCheck if target is a native (eq to host) or not\nTest if this architecture uses the Thumb instruction set.\nCreates a new target given a triple\nThe “operating system” (sometimes also the …\nReturn the pointer bit width of this target’s …\nReturn the pointer bit width of this target’s …\nReturn the pointer width of this target’s architecture.\nRetrieves an empty set of <code>CpuFeature</code>s.\nThe triple associated for the target.\nReturn a <code>Triple</code> with all unknown fields.\nThe “vendor” (whatever that means).\nAn archived <code>CompiledFunctionUnwindInfo</code>\nCompiled function unwind information.\nAny struct that acts like a <code>CompiledFunctionUnwindInfo</code>.\nGeneric reference to data in a <code>CompiledFunctionUnwindInfo</code>\nThe resolver for an archived <code>CompiledFunctionUnwindInfo</code>\nThe resolver for <code>CompiledFunctionUnwindInfo::Dwarf</code>\nThe archived counterpart of …\nThe unwind info is added to the Dwarf section in …\nThe resolver for <code>CompiledFunctionUnwindInfo::WindowsX64</code>\nThe archived counterpart of …\nWindows UNWIND_INFO.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.")