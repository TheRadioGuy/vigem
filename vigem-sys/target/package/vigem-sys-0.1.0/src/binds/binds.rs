
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

pub type size_t = ::std::os::raw::c_ulonglong;
pub type ULONG = ::std::os::raw::c_ulong;
pub type PULONG = *mut ULONG;
pub type USHORT = ::std::os::raw::c_ushort;
pub type DWORD = ::std::os::raw::c_ulong;
pub type BOOL = ::std::os::raw::c_int;
pub type BYTE = ::std::os::raw::c_uchar;
pub type WORD = ::std::os::raw::c_ushort;
pub type LPVOID = *mut ::std::os::raw::c_void;
pub type INT_PTR = ::std::os::raw::c_longlong;
pub type ULONG_PTR = ::std::os::raw::c_ulonglong;
pub type DWORD64 = ::std::os::raw::c_ulonglong;
pub type PVOID = *mut ::std::os::raw::c_void;
pub type SHORT = ::std::os::raw::c_short;
pub type LONG = ::std::os::raw::c_long;
pub type HANDLE = *mut ::std::os::raw::c_void;
pub type LONGLONG = ::std::os::raw::c_longlong;
pub type ULONGLONG = ::std::os::raw::c_ulonglong;
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct _M128A {
    pub Low: ULONGLONG,
    pub High: LONGLONG,
}
#[test]
fn bindgen_test_layout__M128A() {
    assert_eq!(
        ::std::mem::size_of::<_M128A>(),
        16usize,
        concat!("Size of: ", stringify!(_M128A))
    );
    assert_eq!(
        ::std::mem::align_of::<_M128A>(),
        16usize,
        concat!("Alignment of ", stringify!(_M128A))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_M128A>())).Low as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_M128A),
            "::",
            stringify!(Low)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_M128A>())).High as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_M128A),
            "::",
            stringify!(High)
        )
    );
}
pub type M128A = _M128A;
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub struct _XSAVE_FORMAT {
    pub ControlWord: WORD,
    pub StatusWord: WORD,
    pub TagWord: BYTE,
    pub Reserved1: BYTE,
    pub ErrorOpcode: WORD,
    pub ErrorOffset: DWORD,
    pub ErrorSelector: WORD,
    pub Reserved2: WORD,
    pub DataOffset: DWORD,
    pub DataSelector: WORD,
    pub Reserved3: WORD,
    pub MxCsr: DWORD,
    pub MxCsr_Mask: DWORD,
    pub FloatRegisters: [M128A; 8usize],
    pub XmmRegisters: [M128A; 16usize],
    pub Reserved4: [BYTE; 96usize],
}
#[test]
fn bindgen_test_layout__XSAVE_FORMAT() {
    assert_eq!(
        ::std::mem::size_of::<_XSAVE_FORMAT>(),
        512usize,
        concat!("Size of: ", stringify!(_XSAVE_FORMAT))
    );
    assert_eq!(
        ::std::mem::align_of::<_XSAVE_FORMAT>(),
        16usize,
        concat!("Alignment of ", stringify!(_XSAVE_FORMAT))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_XSAVE_FORMAT>())).ControlWord as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSAVE_FORMAT),
            "::",
            stringify!(ControlWord)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_XSAVE_FORMAT>())).StatusWord as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSAVE_FORMAT),
            "::",
            stringify!(StatusWord)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_XSAVE_FORMAT>())).TagWord as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSAVE_FORMAT),
            "::",
            stringify!(TagWord)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_XSAVE_FORMAT>())).Reserved1 as *const _ as usize },
        5usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSAVE_FORMAT),
            "::",
            stringify!(Reserved1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_XSAVE_FORMAT>())).ErrorOpcode as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSAVE_FORMAT),
            "::",
            stringify!(ErrorOpcode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_XSAVE_FORMAT>())).ErrorOffset as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSAVE_FORMAT),
            "::",
            stringify!(ErrorOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_XSAVE_FORMAT>())).ErrorSelector as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSAVE_FORMAT),
            "::",
            stringify!(ErrorSelector)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_XSAVE_FORMAT>())).Reserved2 as *const _ as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSAVE_FORMAT),
            "::",
            stringify!(Reserved2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_XSAVE_FORMAT>())).DataOffset as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSAVE_FORMAT),
            "::",
            stringify!(DataOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_XSAVE_FORMAT>())).DataSelector as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSAVE_FORMAT),
            "::",
            stringify!(DataSelector)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_XSAVE_FORMAT>())).Reserved3 as *const _ as usize },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSAVE_FORMAT),
            "::",
            stringify!(Reserved3)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_XSAVE_FORMAT>())).MxCsr as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSAVE_FORMAT),
            "::",
            stringify!(MxCsr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_XSAVE_FORMAT>())).MxCsr_Mask as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSAVE_FORMAT),
            "::",
            stringify!(MxCsr_Mask)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_XSAVE_FORMAT>())).FloatRegisters as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSAVE_FORMAT),
            "::",
            stringify!(FloatRegisters)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_XSAVE_FORMAT>())).XmmRegisters as *const _ as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSAVE_FORMAT),
            "::",
            stringify!(XmmRegisters)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_XSAVE_FORMAT>())).Reserved4 as *const _ as usize },
        416usize,
        concat!(
            "Offset of field: ",
            stringify!(_XSAVE_FORMAT),
            "::",
            stringify!(Reserved4)
        )
    );
}
pub type XSAVE_FORMAT = _XSAVE_FORMAT;
pub type XMM_SAVE_AREA32 = XSAVE_FORMAT;
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub struct _CONTEXT {
    pub P1Home: DWORD64,
    pub P2Home: DWORD64,
    pub P3Home: DWORD64,
    pub P4Home: DWORD64,
    pub P5Home: DWORD64,
    pub P6Home: DWORD64,
    pub ContextFlags: DWORD,
    pub MxCsr: DWORD,
    pub SegCs: WORD,
    pub SegDs: WORD,
    pub SegEs: WORD,
    pub SegFs: WORD,
    pub SegGs: WORD,
    pub SegSs: WORD,
    pub EFlags: DWORD,
    pub Dr0: DWORD64,
    pub Dr1: DWORD64,
    pub Dr2: DWORD64,
    pub Dr3: DWORD64,
    pub Dr6: DWORD64,
    pub Dr7: DWORD64,
    pub Rax: DWORD64,
    pub Rcx: DWORD64,
    pub Rdx: DWORD64,
    pub Rbx: DWORD64,
    pub Rsp: DWORD64,
    pub Rbp: DWORD64,
    pub Rsi: DWORD64,
    pub Rdi: DWORD64,
    pub R8: DWORD64,
    pub R9: DWORD64,
    pub R10: DWORD64,
    pub R11: DWORD64,
    pub R12: DWORD64,
    pub R13: DWORD64,
    pub R14: DWORD64,
    pub R15: DWORD64,
    pub Rip: DWORD64,
    pub __bindgen_anon_1: _CONTEXT__bindgen_ty_1,
    pub VectorRegister: [M128A; 26usize],
    pub VectorControl: DWORD64,
    pub DebugControl: DWORD64,
    pub LastBranchToRip: DWORD64,
    pub LastBranchFromRip: DWORD64,
    pub LastExceptionToRip: DWORD64,
    pub LastExceptionFromRip: DWORD64,
}
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub union _CONTEXT__bindgen_ty_1 {
    pub FltSave: XMM_SAVE_AREA32,
    pub __bindgen_anon_1: _CONTEXT__bindgen_ty_1__bindgen_ty_1,
    _bindgen_union_align: [u128; 32usize],
}
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct _CONTEXT__bindgen_ty_1__bindgen_ty_1 {
    pub Header: [M128A; 2usize],
    pub Legacy: [M128A; 8usize],
    pub Xmm0: M128A,
    pub Xmm1: M128A,
    pub Xmm2: M128A,
    pub Xmm3: M128A,
    pub Xmm4: M128A,
    pub Xmm5: M128A,
    pub Xmm6: M128A,
    pub Xmm7: M128A,
    pub Xmm8: M128A,
    pub Xmm9: M128A,
    pub Xmm10: M128A,
    pub Xmm11: M128A,
    pub Xmm12: M128A,
    pub Xmm13: M128A,
    pub Xmm14: M128A,
    pub Xmm15: M128A,
}
#[test]
fn bindgen_test_layout__CONTEXT__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<_CONTEXT__bindgen_ty_1__bindgen_ty_1>(),
        416usize,
        concat!(
            "Size of: ",
            stringify!(_CONTEXT__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<_CONTEXT__bindgen_ty_1__bindgen_ty_1>(),
        16usize,
        concat!(
            "Alignment of ",
            stringify!(_CONTEXT__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CONTEXT__bindgen_ty_1__bindgen_ty_1>())).Header as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Header)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CONTEXT__bindgen_ty_1__bindgen_ty_1>())).Legacy as *const _
                as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Legacy)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CONTEXT__bindgen_ty_1__bindgen_ty_1>())).Xmm0 as *const _
                as usize
        },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Xmm0)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CONTEXT__bindgen_ty_1__bindgen_ty_1>())).Xmm1 as *const _
                as usize
        },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Xmm1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CONTEXT__bindgen_ty_1__bindgen_ty_1>())).Xmm2 as *const _
                as usize
        },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Xmm2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CONTEXT__bindgen_ty_1__bindgen_ty_1>())).Xmm3 as *const _
                as usize
        },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Xmm3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CONTEXT__bindgen_ty_1__bindgen_ty_1>())).Xmm4 as *const _
                as usize
        },
        224usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Xmm4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CONTEXT__bindgen_ty_1__bindgen_ty_1>())).Xmm5 as *const _
                as usize
        },
        240usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Xmm5)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CONTEXT__bindgen_ty_1__bindgen_ty_1>())).Xmm6 as *const _
                as usize
        },
        256usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Xmm6)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CONTEXT__bindgen_ty_1__bindgen_ty_1>())).Xmm7 as *const _
                as usize
        },
        272usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Xmm7)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CONTEXT__bindgen_ty_1__bindgen_ty_1>())).Xmm8 as *const _
                as usize
        },
        288usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Xmm8)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CONTEXT__bindgen_ty_1__bindgen_ty_1>())).Xmm9 as *const _
                as usize
        },
        304usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Xmm9)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CONTEXT__bindgen_ty_1__bindgen_ty_1>())).Xmm10 as *const _
                as usize
        },
        320usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Xmm10)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CONTEXT__bindgen_ty_1__bindgen_ty_1>())).Xmm11 as *const _
                as usize
        },
        336usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Xmm11)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CONTEXT__bindgen_ty_1__bindgen_ty_1>())).Xmm12 as *const _
                as usize
        },
        352usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Xmm12)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CONTEXT__bindgen_ty_1__bindgen_ty_1>())).Xmm13 as *const _
                as usize
        },
        368usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Xmm13)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CONTEXT__bindgen_ty_1__bindgen_ty_1>())).Xmm14 as *const _
                as usize
        },
        384usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Xmm14)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CONTEXT__bindgen_ty_1__bindgen_ty_1>())).Xmm15 as *const _
                as usize
        },
        400usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(Xmm15)
        )
    );
}
#[test]
fn bindgen_test_layout__CONTEXT__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<_CONTEXT__bindgen_ty_1>(),
        512usize,
        concat!("Size of: ", stringify!(_CONTEXT__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<_CONTEXT__bindgen_ty_1>(),
        16usize,
        concat!("Alignment of ", stringify!(_CONTEXT__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT__bindgen_ty_1>())).FltSave as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT__bindgen_ty_1),
            "::",
            stringify!(FltSave)
        )
    );
}
#[test]
fn bindgen_test_layout__CONTEXT() {
    assert_eq!(
        ::std::mem::size_of::<_CONTEXT>(),
        1232usize,
        concat!("Size of: ", stringify!(_CONTEXT))
    );
    assert_eq!(
        ::std::mem::align_of::<_CONTEXT>(),
        16usize,
        concat!("Alignment of ", stringify!(_CONTEXT))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).P1Home as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(P1Home)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).P2Home as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(P2Home)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).P3Home as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(P3Home)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).P4Home as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(P4Home)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).P5Home as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(P5Home)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).P6Home as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(P6Home)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).ContextFlags as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(ContextFlags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).MxCsr as *const _ as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(MxCsr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).SegCs as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(SegCs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).SegDs as *const _ as usize },
        58usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(SegDs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).SegEs as *const _ as usize },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(SegEs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).SegFs as *const _ as usize },
        62usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(SegFs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).SegGs as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(SegGs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).SegSs as *const _ as usize },
        66usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(SegSs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).EFlags as *const _ as usize },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(EFlags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).Dr0 as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(Dr0)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).Dr1 as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(Dr1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).Dr2 as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(Dr2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).Dr3 as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(Dr3)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).Dr6 as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(Dr6)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).Dr7 as *const _ as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(Dr7)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).Rax as *const _ as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(Rax)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).Rcx as *const _ as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(Rcx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).Rdx as *const _ as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(Rdx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).Rbx as *const _ as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(Rbx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).Rsp as *const _ as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(Rsp)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).Rbp as *const _ as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(Rbp)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).Rsi as *const _ as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(Rsi)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).Rdi as *const _ as usize },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(Rdi)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).R8 as *const _ as usize },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(R8)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).R9 as *const _ as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(R9)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).R10 as *const _ as usize },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(R10)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).R11 as *const _ as usize },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(R11)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).R12 as *const _ as usize },
        216usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(R12)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).R13 as *const _ as usize },
        224usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(R13)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).R14 as *const _ as usize },
        232usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(R14)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).R15 as *const _ as usize },
        240usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(R15)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).Rip as *const _ as usize },
        248usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(Rip)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).VectorRegister as *const _ as usize },
        768usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(VectorRegister)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).VectorControl as *const _ as usize },
        1184usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(VectorControl)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).DebugControl as *const _ as usize },
        1192usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(DebugControl)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).LastBranchToRip as *const _ as usize },
        1200usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(LastBranchToRip)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).LastBranchFromRip as *const _ as usize },
        1208usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(LastBranchFromRip)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).LastExceptionToRip as *const _ as usize },
        1216usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(LastExceptionToRip)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CONTEXT>())).LastExceptionFromRip as *const _ as usize },
        1224usize,
        concat!(
            "Offset of field: ",
            stringify!(_CONTEXT),
            "::",
            stringify!(LastExceptionFromRip)
        )
    );
}
pub type PCONTEXT = *mut _CONTEXT;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _EXCEPTION_RECORD {
    pub ExceptionCode: DWORD,
    pub ExceptionFlags: DWORD,
    pub ExceptionRecord: *mut _EXCEPTION_RECORD,
    pub ExceptionAddress: PVOID,
    pub NumberParameters: DWORD,
    pub ExceptionInformation: [ULONG_PTR; 15usize],
}
#[test]
fn bindgen_test_layout__EXCEPTION_RECORD() {
    assert_eq!(
        ::std::mem::size_of::<_EXCEPTION_RECORD>(),
        152usize,
        concat!("Size of: ", stringify!(_EXCEPTION_RECORD))
    );
    assert_eq!(
        ::std::mem::align_of::<_EXCEPTION_RECORD>(),
        8usize,
        concat!("Alignment of ", stringify!(_EXCEPTION_RECORD))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_EXCEPTION_RECORD>())).ExceptionCode as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_EXCEPTION_RECORD),
            "::",
            stringify!(ExceptionCode)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_EXCEPTION_RECORD>())).ExceptionFlags as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_EXCEPTION_RECORD),
            "::",
            stringify!(ExceptionFlags)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_EXCEPTION_RECORD>())).ExceptionRecord as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_EXCEPTION_RECORD),
            "::",
            stringify!(ExceptionRecord)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_EXCEPTION_RECORD>())).ExceptionAddress as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_EXCEPTION_RECORD),
            "::",
            stringify!(ExceptionAddress)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_EXCEPTION_RECORD>())).NumberParameters as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_EXCEPTION_RECORD),
            "::",
            stringify!(NumberParameters)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_EXCEPTION_RECORD>())).ExceptionInformation as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_EXCEPTION_RECORD),
            "::",
            stringify!(ExceptionInformation)
        )
    );
}
pub type EXCEPTION_RECORD = _EXCEPTION_RECORD;
pub type PEXCEPTION_RECORD = *mut EXCEPTION_RECORD;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _EXCEPTION_POINTERS {
    pub ExceptionRecord: PEXCEPTION_RECORD,
    pub ContextRecord: PCONTEXT,
}
#[test]
fn bindgen_test_layout__EXCEPTION_POINTERS() {
    assert_eq!(
        ::std::mem::size_of::<_EXCEPTION_POINTERS>(),
        16usize,
        concat!("Size of: ", stringify!(_EXCEPTION_POINTERS))
    );
    assert_eq!(
        ::std::mem::align_of::<_EXCEPTION_POINTERS>(),
        8usize,
        concat!("Alignment of ", stringify!(_EXCEPTION_POINTERS))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_EXCEPTION_POINTERS>())).ExceptionRecord as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_EXCEPTION_POINTERS),
            "::",
            stringify!(ExceptionRecord)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_EXCEPTION_POINTERS>())).ContextRecord as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_EXCEPTION_POINTERS),
            "::",
            stringify!(ContextRecord)
        )
    );
}
pub type FARPROC = ::std::option::Option<unsafe extern "C" fn() -> INT_PTR>;
pub type std_integral_constant_value_type<_Ty> = _Ty;
pub type std_integral_constant_type = u8;
pub type std_true_type = u8;
pub type std_conditional_type<_Ty1> = _Ty1;
pub type std_conditional_t = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_input_iterator_tag {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std_input_iterator_tag() {
    assert_eq!(
        ::std::mem::size_of::<std_input_iterator_tag>(),
        1usize,
        concat!("Size of: ", stringify!(std_input_iterator_tag))
    );
    assert_eq!(
        ::std::mem::align_of::<std_input_iterator_tag>(),
        1usize,
        concat!("Alignment of ", stringify!(std_input_iterator_tag))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_forward_iterator_tag {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std_forward_iterator_tag() {
    assert_eq!(
        ::std::mem::size_of::<std_forward_iterator_tag>(),
        1usize,
        concat!("Size of: ", stringify!(std_forward_iterator_tag))
    );
    assert_eq!(
        ::std::mem::align_of::<std_forward_iterator_tag>(),
        1usize,
        concat!("Alignment of ", stringify!(std_forward_iterator_tag))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_bidirectional_iterator_tag {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std_bidirectional_iterator_tag() {
    assert_eq!(
        ::std::mem::size_of::<std_bidirectional_iterator_tag>(),
        1usize,
        concat!("Size of: ", stringify!(std_bidirectional_iterator_tag))
    );
    assert_eq!(
        ::std::mem::align_of::<std_bidirectional_iterator_tag>(),
        1usize,
        concat!("Alignment of ", stringify!(std_bidirectional_iterator_tag))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_random_access_iterator_tag {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std_random_access_iterator_tag() {
    assert_eq!(
        ::std::mem::size_of::<std_random_access_iterator_tag>(),
        1usize,
        concat!("Size of: ", stringify!(std_random_access_iterator_tag))
    );
    assert_eq!(
        ::std::mem::align_of::<std_random_access_iterator_tag>(),
        1usize,
        concat!("Alignment of ", stringify!(std_random_access_iterator_tag))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std__Iterator_traits_base {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_iterator_traits {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_reverse_iterator<_BidIt> {
    pub current: _BidIt,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_BidIt>>,
}
pub type std_reverse_iterator_iterator_type<_BidIt> = _BidIt;
pub type std_reverse_iterator_iterator_category = std_iterator_traits;
pub type std_reverse_iterator_value_type = std_iterator_traits;
pub type std_reverse_iterator_difference_type = std_iterator_traits;
pub type std_reverse_iterator_pointer = std_iterator_traits;
pub type std_reverse_iterator_reference = std_iterator_traits;
pub type std_reverse_iterator__Prevent_inheriting_unwrap<_BidIt> = std_reverse_iterator<_BidIt>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_allocator_traits {
    pub _address: u8,
}
pub type std__Rebind_alloc_t = std_allocator_traits;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_allocator {
    pub _address: u8,
}
pub type std_allocator__From_primary = std_allocator;
pub type std_allocator_value_type<_Ty> = _Ty;
pub type std_allocator_pointer<_Ty> = *mut _Ty;
pub type std_allocator_const_pointer<_Ty> = *const _Ty;
pub type std_allocator_reference<_Ty> = *mut _Ty;
pub type std_allocator_const_reference<_Ty> = *const _Ty;
pub type std_allocator_size_type = size_t;
pub type std_allocator_difference_type = isize;
pub type std_allocator_propagate_on_container_move_assignment = std_true_type;
pub type std_allocator_is_always_equal = std_true_type;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_allocator_rebind {
    pub _address: u8,
}
pub type std_allocator_rebind_other = std_allocator;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std__Container_base0 {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std__Container_base0() {
    assert_eq!(
        ::std::mem::size_of::<std__Container_base0>(),
        1usize,
        concat!("Size of: ", stringify!(std__Container_base0))
    );
    assert_eq!(
        ::std::mem::align_of::<std__Container_base0>(),
        1usize,
        concat!("Alignment of ", stringify!(std__Container_base0))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std__Iterator_base0 {
    pub _address: u8,
}
pub const std__Iterator_base0__Unwrap_when_unverified: bool = true;
#[test]
fn bindgen_test_layout_std__Iterator_base0() {
    assert_eq!(
        ::std::mem::size_of::<std__Iterator_base0>(),
        1usize,
        concat!("Size of: ", stringify!(std__Iterator_base0))
    );
    assert_eq!(
        ::std::mem::align_of::<std__Iterator_base0>(),
        1usize,
        concat!("Alignment of ", stringify!(std__Iterator_base0))
    );
}
pub type std__Container_base = std__Container_base0;
pub type std__Iterator_base = std__Iterator_base0;
pub type std__Compressed_pair__Mybase<_Ty1> = _Ty1;
#[repr(C)]
pub struct std__Vector_const_iterator {
    pub _Ptr: std__Vector_const_iterator__Tptr,
}
pub type std__Vector_const_iterator_iterator_category = std_random_access_iterator_tag;
pub type std__Vector_const_iterator_value_type = [u8; 0usize];
pub type std__Vector_const_iterator_difference_type = [u8; 0usize];
pub type std__Vector_const_iterator_pointer = [u8; 0usize];
pub type std__Vector_const_iterator_reference = *const std__Vector_const_iterator_value_type;
pub type std__Vector_const_iterator__Tptr = [u8; 0usize];
pub type std__Vector_const_iterator__Prevent_inheriting_unwrap = std__Vector_const_iterator;
#[repr(C)]
pub struct std__Vector_iterator {
    pub _base: std__Vector_const_iterator,
}
pub type std__Vector_iterator__Mybase = std__Vector_const_iterator;
pub type std__Vector_iterator_iterator_category = std_random_access_iterator_tag;
pub type std__Vector_iterator_value_type = [u8; 0usize];
pub type std__Vector_iterator_difference_type = [u8; 0usize];
pub type std__Vector_iterator_pointer = [u8; 0usize];
pub type std__Vector_iterator_reference = *mut std__Vector_iterator_value_type;
pub type std__Vector_iterator__Prevent_inheriting_unwrap = std__Vector_iterator;
#[repr(C)]
pub struct std__Vector_val {
    pub _Myfirst: std__Vector_val_pointer,
    pub _Mylast: std__Vector_val_pointer,
    pub _Myend: std__Vector_val_pointer,
}
pub type std__Vector_val_value_type = [u8; 0usize];
pub type std__Vector_val_size_type = [u8; 0usize];
pub type std__Vector_val_difference_type = [u8; 0usize];
pub type std__Vector_val_pointer = [u8; 0usize];
pub type std__Vector_val_const_pointer = [u8; 0usize];
pub type std__Vector_val_reference = *mut std__Vector_val_value_type;
pub type std__Vector_val_const_reference = *const std__Vector_val_value_type;
#[repr(C)]
#[derive(Debug)]
pub struct std_vector {
    pub _Mypair: u8,
}
pub type std_vector__Alty = std__Rebind_alloc_t;
pub type std_vector__Alty_traits = std_allocator_traits;
pub type std_vector_value_type<_Ty> = _Ty;
pub type std_vector_allocator_type<_Alloc> = _Alloc;
pub type std_vector_pointer = std_vector__Alty_traits;
pub type std_vector_const_pointer = std_vector__Alty_traits;
pub type std_vector_reference<_Ty> = *mut _Ty;
pub type std_vector_const_reference<_Ty> = *const _Ty;
pub type std_vector_size_type = std_vector__Alty_traits;
pub type std_vector_difference_type = std_vector__Alty_traits;
pub type std_vector__Scary_val = std__Vector_val;
pub type std_vector_iterator = std__Vector_iterator;
pub type std_vector_const_iterator = std__Vector_const_iterator;
pub type std_vector_reverse_iterator = std_reverse_iterator<std_vector_iterator>;
pub type std_vector_const_reverse_iterator = std_reverse_iterator<std_vector_const_iterator>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_default_delete {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std__Get_deleter_pointer_type {
    pub _address: u8,
}
pub type std__Get_deleter_pointer_type_type<_Ty> = *mut _Ty;
#[repr(C)]
#[derive(Debug)]
pub struct std_unique_ptr {
    pub _Mypair: u8,
}
pub type std_unique_ptr_pointer = std__Get_deleter_pointer_type;
pub type std_unique_ptr_element_type<_Ty> = _Ty;
pub type std_unique_ptr_deleter_type<_Dx> = _Dx;
#[repr(C)]
#[derive(Debug)]
pub struct std_thread {
    pub _Thr: _Thrd_t,
}
pub type std_thread_native_handle_type = *mut ::std::os::raw::c_void;
#[test]
fn bindgen_test_layout_std_thread() {
    assert_eq!(
        ::std::mem::size_of::<std_thread>(),
        16usize,
        concat!("Size of: ", stringify!(std_thread))
    );
    assert_eq!(
        ::std::mem::align_of::<std_thread>(),
        8usize,
        concat!("Alignment of ", stringify!(std_thread))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<std_thread>()))._Thr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(std_thread),
            "::",
            stringify!(_Thr)
        )
    );
}
extern "C" {
    #[link_name = "\u{1}?get_id@thread@std@@QEBA?AVid@12@XZ"]
    pub fn std_thread_get_id(this: *const std_thread) -> std_thread_id;
}
impl std_thread {
    #[inline]
    pub unsafe fn get_id(&self) -> std_thread_id {
        std_thread_get_id(self)
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_thread_id {
    pub _Id: _Thrd_id_t,
}
#[test]
fn bindgen_test_layout_std_thread_id() {
    assert_eq!(
        ::std::mem::size_of::<std_thread_id>(),
        4usize,
        concat!("Size of: ", stringify!(std_thread_id))
    );
    assert_eq!(
        ::std::mem::align_of::<std_thread_id>(),
        4usize,
        concat!("Alignment of ", stringify!(std_thread_id))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<std_thread_id>()))._Id as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(std_thread_id),
            "::",
            stringify!(_Id)
        )
    );
}
pub const _VIGEM_TARGET_TYPE_Xbox360Wired: _VIGEM_TARGET_TYPE = 0;
pub const _VIGEM_TARGET_TYPE_DualShock4Wired: _VIGEM_TARGET_TYPE = 2;
pub type _VIGEM_TARGET_TYPE = i32;
pub use self::_VIGEM_TARGET_TYPE as VIGEM_TARGET_TYPE;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XUSB_REPORT {
    pub wButtons: USHORT,
    pub bLeftTrigger: BYTE,
    pub bRightTrigger: BYTE,
    pub sThumbLX: SHORT,
    pub sThumbLY: SHORT,
    pub sThumbRX: SHORT,
    pub sThumbRY: SHORT,
}
#[test]
fn bindgen_test_layout__XUSB_REPORT() {
    assert_eq!(
        ::std::mem::size_of::<_XUSB_REPORT>(),
        12usize,
        concat!("Size of: ", stringify!(_XUSB_REPORT))
    );
    assert_eq!(
        ::std::mem::align_of::<_XUSB_REPORT>(),
        2usize,
        concat!("Alignment of ", stringify!(_XUSB_REPORT))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_XUSB_REPORT>())).wButtons as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_XUSB_REPORT),
            "::",
            stringify!(wButtons)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_XUSB_REPORT>())).bLeftTrigger as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(_XUSB_REPORT),
            "::",
            stringify!(bLeftTrigger)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_XUSB_REPORT>())).bRightTrigger as *const _ as usize },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(_XUSB_REPORT),
            "::",
            stringify!(bRightTrigger)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_XUSB_REPORT>())).sThumbLX as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_XUSB_REPORT),
            "::",
            stringify!(sThumbLX)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_XUSB_REPORT>())).sThumbLY as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(_XUSB_REPORT),
            "::",
            stringify!(sThumbLY)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_XUSB_REPORT>())).sThumbRX as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_XUSB_REPORT),
            "::",
            stringify!(sThumbRX)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_XUSB_REPORT>())).sThumbRY as *const _ as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(_XUSB_REPORT),
            "::",
            stringify!(sThumbRY)
        )
    );
}
pub type XUSB_REPORT = _XUSB_REPORT;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _DS4_REPORT {
    pub bThumbLX: BYTE,
    pub bThumbLY: BYTE,
    pub bThumbRX: BYTE,
    pub bThumbRY: BYTE,
    pub wButtons: USHORT,
    pub bSpecial: BYTE,
    pub bTriggerL: BYTE,
    pub bTriggerR: BYTE,
}
#[test]
fn bindgen_test_layout__DS4_REPORT() {
    assert_eq!(
        ::std::mem::size_of::<_DS4_REPORT>(),
        10usize,
        concat!("Size of: ", stringify!(_DS4_REPORT))
    );
    assert_eq!(
        ::std::mem::align_of::<_DS4_REPORT>(),
        2usize,
        concat!("Alignment of ", stringify!(_DS4_REPORT))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_DS4_REPORT>())).bThumbLX as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_DS4_REPORT),
            "::",
            stringify!(bThumbLX)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_DS4_REPORT>())).bThumbLY as *const _ as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(_DS4_REPORT),
            "::",
            stringify!(bThumbLY)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_DS4_REPORT>())).bThumbRX as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(_DS4_REPORT),
            "::",
            stringify!(bThumbRX)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_DS4_REPORT>())).bThumbRY as *const _ as usize },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(_DS4_REPORT),
            "::",
            stringify!(bThumbRY)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_DS4_REPORT>())).wButtons as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_DS4_REPORT),
            "::",
            stringify!(wButtons)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_DS4_REPORT>())).bSpecial as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(_DS4_REPORT),
            "::",
            stringify!(bSpecial)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_DS4_REPORT>())).bTriggerL as *const _ as usize },
        7usize,
        concat!(
            "Offset of field: ",
            stringify!(_DS4_REPORT),
            "::",
            stringify!(bTriggerL)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_DS4_REPORT>())).bTriggerR as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_DS4_REPORT),
            "::",
            stringify!(bTriggerR)
        )
    );
}
pub type DS4_REPORT = _DS4_REPORT;
pub const _VIGEM_ERRORS_VIGEM_ERROR_NONE: _VIGEM_ERRORS = 536870912;
pub const _VIGEM_ERRORS_VIGEM_ERROR_BUS_NOT_FOUND: _VIGEM_ERRORS = -536870911;
pub const _VIGEM_ERRORS_VIGEM_ERROR_NO_FREE_SLOT: _VIGEM_ERRORS = -536870910;
pub const _VIGEM_ERRORS_VIGEM_ERROR_INVALID_TARGET: _VIGEM_ERRORS = -536870909;
pub const _VIGEM_ERRORS_VIGEM_ERROR_REMOVAL_FAILED: _VIGEM_ERRORS = -536870908;
pub const _VIGEM_ERRORS_VIGEM_ERROR_ALREADY_CONNECTED: _VIGEM_ERRORS = -536870907;
pub const _VIGEM_ERRORS_VIGEM_ERROR_TARGET_UNINITIALIZED: _VIGEM_ERRORS = -536870906;
pub const _VIGEM_ERRORS_VIGEM_ERROR_TARGET_NOT_PLUGGED_IN: _VIGEM_ERRORS = -536870905;
pub const _VIGEM_ERRORS_VIGEM_ERROR_BUS_VERSION_MISMATCH: _VIGEM_ERRORS = -536870904;
pub const _VIGEM_ERRORS_VIGEM_ERROR_BUS_ACCESS_FAILED: _VIGEM_ERRORS = -536870903;
pub const _VIGEM_ERRORS_VIGEM_ERROR_CALLBACK_ALREADY_REGISTERED: _VIGEM_ERRORS = -536870896;
pub const _VIGEM_ERRORS_VIGEM_ERROR_CALLBACK_NOT_FOUND: _VIGEM_ERRORS = -536870895;
pub const _VIGEM_ERRORS_VIGEM_ERROR_BUS_ALREADY_CONNECTED: _VIGEM_ERRORS = -536870894;
pub const _VIGEM_ERRORS_VIGEM_ERROR_BUS_INVALID_HANDLE: _VIGEM_ERRORS = -536870893;
pub const _VIGEM_ERRORS_VIGEM_ERROR_XUSB_USERINDEX_OUT_OF_RANGE: _VIGEM_ERRORS = -536870892;
pub const _VIGEM_ERRORS_VIGEM_ERROR_INVALID_PARAMETER: _VIGEM_ERRORS = -536870891;
#[doc = " \\typedef enum _VIGEM_ERRORS"]
#[doc = ""]
#[doc = " \\brief   Defines an alias representing the ViGEm errors."]
pub type _VIGEM_ERRORS = i32;
#[doc = " \\typedef enum _VIGEM_ERRORS"]
#[doc = ""]
#[doc = " \\brief   Defines an alias representing the ViGEm errors."]
pub use self::_VIGEM_ERRORS as VIGEM_ERROR;
#[doc = " \\typedef struct _VIGEM_CLIENT_T *PVIGEM_CLIENT"]
#[doc = ""]
#[doc = " \\brief   Defines an alias representing a driver connection object."]
pub type PVIGEM_CLIENT = *mut _VIGEM_CLIENT_T;
#[doc = " \\typedef struct _VIGEM_TARGET_T *PVIGEM_TARGET"]
#[doc = ""]
#[doc = " \\brief   Defines an alias representing a target device object."]
pub type PVIGEM_TARGET = *mut _VIGEM_TARGET_T;
pub type PFN_VIGEM_TARGET_ADD_RESULT = ::std::option::Option<unsafe extern "C" fn()>;
pub type PFN_VIGEM_X360_NOTIFICATION = ::std::option::Option<unsafe extern "C" fn()>;
pub type PFN_VIGEM_DS4_NOTIFICATION = ::std::option::Option<unsafe extern "C" fn()>;
extern "C" {
    #[doc = " \\fn  PVIGEM_CLIENT vigem_alloc(void);"]
    #[doc = ""]
    #[doc = " \\brief   Allocates an object representing a driver connection."]
    #[doc = ""]
    #[doc = " \\author  Benjamin \"Nefarius\" H�glinger"]
    #[doc = " \\date    28.08.2017"]
    #[doc = ""]
    #[doc = " \\return  A new driver connection object."]
    pub fn vigem_alloc() -> PVIGEM_CLIENT;
}
extern "C" {
    #[doc = " \\fn  void vigem_free(PVIGEM_CLIENT vigem);"]
    #[doc = ""]
    #[doc = " \\brief   Frees up memory used by the driver connection object."]
    #[doc = ""]
    #[doc = " \\author  Benjamin \"Nefarius\" H�glinger"]
    #[doc = " \\date    28.08.2017"]
    #[doc = ""]
    #[doc = " \\param   vigem   The driver connection object."]
    pub fn vigem_free(vigem: PVIGEM_CLIENT);
}
extern "C" {
    #[doc = " \\fn  VIGEM_ERROR vigem_connect(PVIGEM_CLIENT vigem);"]
    #[doc = ""]
    #[doc = " \\brief   Initializes the driver object and establishes a connection to the emulation bus"]
    #[doc = "          driver. Returns an error if no compatible bus device has been found."]
    #[doc = ""]
    #[doc = " \\author  Benjamin \"Nefarius\" H�glinger"]
    #[doc = " \\date    28.08.2017"]
    #[doc = ""]
    #[doc = " \\param   vigem   The driver connection object."]
    #[doc = ""]
    #[doc = " \\return  A VIGEM_ERROR."]
    pub fn vigem_connect(vigem: PVIGEM_CLIENT) -> VIGEM_ERROR;
}
extern "C" {
    #[doc = " \\fn  void vigem_disconnect(PVIGEM_CLIENT vigem);"]
    #[doc = ""]
    #[doc = " \\brief   Disconnects from the bus device and resets the driver object state. The driver object"]
    #[doc = "          may be reused again after calling this function. When called, all targets which may"]
    #[doc = "          still be connected will be destroyed automatically. Be aware, that allocated target"]
    #[doc = "          objects won't be automatically freed, this has to be taken care of by the caller."]
    #[doc = ""]
    #[doc = " \\author  Benjamin \"Nefarius\" H�glinger"]
    #[doc = " \\date    28.08.2017"]
    #[doc = ""]
    #[doc = " \\param   vigem   The driver connection object."]
    pub fn vigem_disconnect(vigem: PVIGEM_CLIENT);
}
extern "C" {
    #[doc = " \\fn  PVIGEM_TARGET vigem_target_x360_alloc(void);"]
    #[doc = ""]
    #[doc = " \\brief   Allocates an object representing an Xbox 360 Controller device."]
    #[doc = ""]
    #[doc = " \\author  Benjamin \"Nefarius\" H�glinger"]
    #[doc = " \\date    28.08.2017"]
    #[doc = ""]
    #[doc = " \\return  A PVIGEM_TARGET representing an Xbox 360 Controller device."]
    pub fn vigem_target_x360_alloc() -> PVIGEM_TARGET;
}
extern "C" {
    #[doc = " \\fn  PVIGEM_TARGET vigem_target_ds4_alloc(void);"]
    #[doc = ""]
    #[doc = " \\brief   Allocates an object representing a DualShock 4 Controller device."]
    #[doc = ""]
    #[doc = " \\author  Benjamin \"Nefarius\" H�glinger"]
    #[doc = " \\date    28.08.2017"]
    #[doc = ""]
    #[doc = " \\return  A PVIGEM_TARGET representing a DualShock 4 Controller device."]
    pub fn vigem_target_ds4_alloc() -> PVIGEM_TARGET;
}
extern "C" {
    #[doc = " \\fn  void vigem_target_free(PVIGEM_TARGET target);"]
    #[doc = ""]
    #[doc = " \\brief   Frees up memory used by the target device object. This does not automatically remove"]
    #[doc = "          the associated device from the bus, if present. If the target device doesn't get"]
    #[doc = "          removed before this call, the device becomes orphaned until the owning process is"]
    #[doc = "          terminated."]
    #[doc = ""]
    #[doc = " \\author  Benjamin \"Nefarius\" H�glinger"]
    #[doc = " \\date    28.08.2017"]
    #[doc = ""]
    #[doc = " \\param   target  The target device object."]
    pub fn vigem_target_free(target: PVIGEM_TARGET);
}
extern "C" {
    #[doc = " \\fn  VIGEM_ERROR vigem_target_add(PVIGEM_CLIENT vigem, PVIGEM_TARGET target);"]
    #[doc = ""]
    #[doc = " \\brief   Adds a provided target device to the bus driver, which is equal to a device plug-in"]
    #[doc = "          event of a physical hardware device. This function blocks until the target device is"]
    #[doc = "          in full operational mode."]
    #[doc = ""]
    #[doc = " \\author  Benjamin \"Nefarius\" H�glinger"]
    #[doc = " \\date    28.08.2017"]
    #[doc = ""]
    #[doc = " \\param   vigem   The driver connection object."]
    #[doc = " \\param   target  The target device object."]
    #[doc = ""]
    #[doc = " \\return  A VIGEM_ERROR."]
    pub fn vigem_target_add(vigem: PVIGEM_CLIENT, target: PVIGEM_TARGET) -> VIGEM_ERROR;
}
extern "C" {
    #[doc = " \\fn  VIGEM_ERROR vigem_target_add_async(PVIGEM_CLIENT vigem, PVIGEM_TARGET target, PVIGEM_TARGET_ADD_RESULT result);"]
    #[doc = ""]
    #[doc = " \\brief   Adds a provided target device to the bus driver, which is equal to a device plug-in"]
    #[doc = "          event of a physical hardware device. This function immediately returns. An optional"]
    #[doc = "          callback may be registered which gets called on error or if the target device has"]
    #[doc = "          become fully operational."]
    #[doc = ""]
    #[doc = " \\author  Benjamin \"Nefarius\" H�glinger"]
    #[doc = " \\date    28.08.2017"]
    #[doc = ""]
    #[doc = " \\param   vigem   The driver connection object."]
    #[doc = " \\param   target  The target device object."]
    #[doc = " \\param   result  An optional function getting called when the target device becomes available."]
    #[doc = ""]
    #[doc = " \\return  A VIGEM_ERROR."]
    pub fn vigem_target_add_async(
        vigem: PVIGEM_CLIENT,
        target: PVIGEM_TARGET,
        result: PFN_VIGEM_TARGET_ADD_RESULT,
    ) -> VIGEM_ERROR;
}
extern "C" {
    #[doc = " \\fn  VIGEM_ERROR vigem_target_remove(PVIGEM_CLIENT vigem, PVIGEM_TARGET target);"]
    #[doc = ""]
    #[doc = " \\brief   Removes a provided target device from the bus driver, which is equal to a device"]
    #[doc = "          unplug event of a physical hardware device. The target device object may be reused"]
    #[doc = "          after this function is called. If this function is never called on target device"]
    #[doc = "          objects, they will be removed from the bus when the owning process terminates."]
    #[doc = ""]
    #[doc = " \\author  Benjamin \"Nefarius\" H�glinger"]
    #[doc = " \\date    28.08.2017"]
    #[doc = ""]
    #[doc = " \\param   vigem   The driver connection object."]
    #[doc = " \\param   target  The target device object."]
    #[doc = ""]
    #[doc = " \\return  A VIGEM_ERROR."]
    pub fn vigem_target_remove(vigem: PVIGEM_CLIENT, target: PVIGEM_TARGET) -> VIGEM_ERROR;
}
extern "C" {
    #[doc = " \\fn  VIGEM_ERROR vigem_target_x360_register_notification(PVIGEM_CLIENT vigem, PVIGEM_TARGET target, PVIGEM_X360_NOTIFICATION notification);"]
    #[doc = ""]
    #[doc = " \\brief   Registers a function which gets called, when LED index or vibration state changes"]
    #[doc = "          occur on the provided target device. This function fails if the provided target"]
    #[doc = "          device isn't fully operational or in an erroneous state."]
    #[doc = ""]
    #[doc = " \\author  Benjamin \"Nefarius\" H�glinger"]
    #[doc = " \\date    28.08.2017"]
    #[doc = ""]
    #[doc = " \\param   vigem           The driver connection object."]
    #[doc = " \\param   target          The target device object."]
    #[doc = " \\param   notification    The notification callback."]
    #[doc = " \\param   userData        The user data passed to the notification callback."]
    #[doc = ""]
    #[doc = " \\return  A VIGEM_ERROR."]
    pub fn vigem_target_x360_register_notification(
        vigem: PVIGEM_CLIENT,
        target: PVIGEM_TARGET,
        notification: PFN_VIGEM_X360_NOTIFICATION,
        userData: LPVOID,
    ) -> VIGEM_ERROR;
}
extern "C" {
    #[doc = " \\fn  VIGEM_ERROR vigem_target_ds4_register_notification(PVIGEM_CLIENT vigem, PVIGEM_TARGET target, PVIGEM_DS4_NOTIFICATION notification);"]
    #[doc = ""]
    #[doc = " \\brief   Registers a function which gets called, when LightBar or vibration state changes"]
    #[doc = "          occur on the provided target device. This function fails if the provided target"]
    #[doc = "          device isn't fully operational or in an erroneous state."]
    #[doc = ""]
    #[doc = " \\author  Benjamin \"Nefarius\" H�glinger"]
    #[doc = " \\date    28.08.2017"]
    #[doc = ""]
    #[doc = " \\param   vigem           The driver connection object."]
    #[doc = " \\param   target          The target device object."]
    #[doc = " \\param   notification    The notification callback."]
    #[doc = " \\param   userData        The user data passed to the notification callback."]
    #[doc = ""]
    #[doc = " \\return  A VIGEM_ERROR."]
    pub fn vigem_target_ds4_register_notification(
        vigem: PVIGEM_CLIENT,
        target: PVIGEM_TARGET,
        notification: PFN_VIGEM_DS4_NOTIFICATION,
        userData: LPVOID,
    ) -> VIGEM_ERROR;
}
extern "C" {
    #[doc = " \\fn  void vigem_target_x360_unregister_notification(PVIGEM_TARGET target);"]
    #[doc = ""]
    #[doc = " \\brief   Removes a previously registered callback function from the provided target object."]
    #[doc = ""]
    #[doc = " \\author  Benjamin \"Nefarius\" H�glinger"]
    #[doc = " \\date    28.08.2017"]
    #[doc = ""]
    #[doc = " \\param   target  The target device object."]
    pub fn vigem_target_x360_unregister_notification(target: PVIGEM_TARGET);
}
extern "C" {
    #[doc = " \\fn  void vigem_target_ds4_unregister_notification(PVIGEM_TARGET target);"]
    #[doc = ""]
    #[doc = " \\brief   Removes a previously registered callback function from the provided target object."]
    #[doc = ""]
    #[doc = " \\author  Benjamin \"Nefarius\" H�glinger"]
    #[doc = " \\date    28.08.2017"]
    #[doc = ""]
    #[doc = " \\param   target  The target device object."]
    pub fn vigem_target_ds4_unregister_notification(target: PVIGEM_TARGET);
}
extern "C" {
    #[doc = " \\fn  void vigem_target_set_vid(PVIGEM_TARGET target, USHORT vid);"]
    #[doc = ""]
    #[doc = " \\brief   Overrides the default Vendor ID value with the provided one."]
    #[doc = ""]
    #[doc = " \\author  Benjamin \"Nefarius\" H�glinger"]
    #[doc = " \\date    28.08.2017"]
    #[doc = ""]
    #[doc = " \\param   target  The target device object."]
    #[doc = " \\param   vid     The Vendor ID to set."]
    pub fn vigem_target_set_vid(target: PVIGEM_TARGET, vid: USHORT);
}
extern "C" {
    #[doc = " \\fn  void vigem_target_set_pid(PVIGEM_TARGET target, USHORT pid);"]
    #[doc = ""]
    #[doc = " \\brief   Overrides the default Product ID value with the provided one."]
    #[doc = ""]
    #[doc = " \\author  Benjamin \"Nefarius\" H�glinger"]
    #[doc = " \\date    28.08.2017"]
    #[doc = ""]
    #[doc = " \\param   target  The target device object."]
    #[doc = " \\param   pid     The Product ID to set."]
    pub fn vigem_target_set_pid(target: PVIGEM_TARGET, pid: USHORT);
}
extern "C" {
    #[doc = " \\fn  USHORT vigem_target_get_vid(PVIGEM_TARGET target);"]
    #[doc = ""]
    #[doc = " \\brief   Returns the Vendor ID of the provided target device object."]
    #[doc = ""]
    #[doc = " \\author  Benjamin \"Nefarius\" H�glinger"]
    #[doc = " \\date    28.08.2017"]
    #[doc = ""]
    #[doc = " \\param   target  The target device object."]
    #[doc = ""]
    #[doc = " \\return  The Vendor ID."]
    pub fn vigem_target_get_vid(target: PVIGEM_TARGET) -> USHORT;
}
extern "C" {
    #[doc = " \\fn  USHORT vigem_target_get_pid(PVIGEM_TARGET target);"]
    #[doc = ""]
    #[doc = " \\brief   Returns the Product ID of the provided target device object."]
    #[doc = ""]
    #[doc = " \\author  Benjamin \"Nefarius\" H�glinger"]
    #[doc = " \\date    28.08.2017"]
    #[doc = ""]
    #[doc = " \\param   target  The target device object."]
    #[doc = ""]
    #[doc = " \\return  The Product ID."]
    pub fn vigem_target_get_pid(target: PVIGEM_TARGET) -> USHORT;
}
extern "C" {
    #[doc = " \\fn  VIGEM_ERROR vigem_target_x360_update(PVIGEM_CLIENT vigem, PVIGEM_TARGET target, XUSB_REPORT report);"]
    #[doc = ""]
    #[doc = " \\brief   Sends a state report to the provided target device."]
    #[doc = ""]
    #[doc = " \\author  Benjamin \"Nefarius\" H�glinger"]
    #[doc = " \\date    28.08.2017"]
    #[doc = ""]
    #[doc = " \\param   vigem   The driver connection object."]
    #[doc = " \\param   target  The target device object."]
    #[doc = " \\param   report  The report to send to the target device."]
    #[doc = ""]
    #[doc = " \\return  A VIGEM_ERROR."]
    pub fn vigem_target_x360_update(
        vigem: PVIGEM_CLIENT,
        target: PVIGEM_TARGET,
        report: XUSB_REPORT,
    ) -> VIGEM_ERROR;
}
extern "C" {
    #[doc = " \\fn  VIGEM_ERROR vigem_target_ds4_update(PVIGEM_CLIENT vigem, PVIGEM_TARGET target, DS4_REPORT report);"]
    #[doc = ""]
    #[doc = " \\brief   Sends a state report to the provided target device."]
    #[doc = ""]
    #[doc = " \\author  Benjamin \"Nefarius\" H�glinger"]
    #[doc = " \\date    28.08.2017"]
    #[doc = ""]
    #[doc = " \\param   vigem   The driver connection object."]
    #[doc = " \\param   target  The target device object."]
    #[doc = " \\param   report  The report to send to the target device."]
    #[doc = ""]
    #[doc = " \\return  A VIGEM_ERROR."]
    pub fn vigem_target_ds4_update(
        vigem: PVIGEM_CLIENT,
        target: PVIGEM_TARGET,
        report: DS4_REPORT,
    ) -> VIGEM_ERROR;
}
extern "C" {
    #[doc = " \\fn  ULONG vigem_target_get_index(PVIGEM_TARGET target);"]
    #[doc = ""]
    #[doc = " \\brief   Returns the internal index (serial number) the bus driver assigned to the provided"]
    #[doc = "          target device object. Note that this value is specific to the inner workings of the"]
    #[doc = "          bus driver, it does not reflect related values like player index or device arrival"]
    #[doc = "          order experienced by other APIs. It may be used to identify the target device object"]
    #[doc = "          for its lifetime. This value becomes invalid once the target device is removed from"]
    #[doc = "          the bus and may change on the next addition of the device."]
    #[doc = ""]
    #[doc = " \\author  Benjamin \"Nefarius\" H�glinger"]
    #[doc = " \\date    28.08.2017"]
    #[doc = ""]
    #[doc = " \\param   target  The target device object."]
    #[doc = ""]
    #[doc = " \\return  The internally used index of the target device."]
    pub fn vigem_target_get_index(target: PVIGEM_TARGET) -> ULONG;
}
extern "C" {
    #[doc = " \\fn  VIGEM_TARGET_TYPE vigem_target_get_type(PVIGEM_TARGET target);"]
    #[doc = ""]
    #[doc = " \\brief   Returns the type of the provided target device object."]
    #[doc = ""]
    #[doc = " \\author  Benjamin \"Nefarius\" H�glinger"]
    #[doc = " \\date    28.08.2017"]
    #[doc = ""]
    #[doc = " \\param   target  The target device object."]
    #[doc = ""]
    #[doc = " \\return  A VIGEM_TARGET_TYPE."]
    pub fn vigem_target_get_type(target: PVIGEM_TARGET) -> VIGEM_TARGET_TYPE;
}
extern "C" {
    #[doc = " \\fn  BOOL vigem_target_is_attached(PVIGEM_TARGET target);"]
    #[doc = ""]
    #[doc = " \\brief   Returns TRUE if the provided target device object is currently attached to the bus,"]
    #[doc = "          FALSE otherwise."]
    #[doc = ""]
    #[doc = " \\author  Benjamin \"Nefarius\" H�glinger"]
    #[doc = " \\date    30.08.2017"]
    #[doc = ""]
    #[doc = " \\param   target  The target device object."]
    #[doc = ""]
    #[doc = " \\return  TRUE if device is attached to the bus, FALSE otherwise."]
    pub fn vigem_target_is_attached(target: PVIGEM_TARGET) -> BOOL;
}
extern "C" {
    #[doc = " \\fn  VIGEM_API VIGEM_ERROR vigem_target_x360_get_user_index(PVIGEM_CLIENT vigem, PVIGEM_TARGET target, PULONG index);"]
    #[doc = ""]
    #[doc = " \\brief   Returns the user index of the emulated Xenon device. This value correspondents to the"]
    #[doc = "          (zero-based) index number representing the player number via LED present on a"]
    #[doc = "          physical controller and is compatible to the dwUserIndex propery of the XInput* APIs."]
    #[doc = ""]
    #[doc = " \\author  Benjamin \"Nefarius\" H�glinger"]
    #[doc = " \\date    10.05.2018"]
    #[doc = ""]
    #[doc = " \\param   vigem   The driver connection object."]
    #[doc = " \\param   target  The target device object."]
    #[doc = " \\param   index   The (zero-based) user index of the Xenon device."]
    #[doc = ""]
    #[doc = " \\return  A VIGEM_ERROR."]
    pub fn vigem_target_x360_get_user_index(
        vigem: PVIGEM_CLIENT,
        target: PVIGEM_TARGET,
        index: PULONG,
    ) -> VIGEM_ERROR;
}
pub type _Thrd_id_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Thrd_t {
    pub _Hnd: *mut ::std::os::raw::c_void,
    pub _Id: _Thrd_id_t,
}
#[test]
fn bindgen_test_layout__Thrd_t() {
    assert_eq!(
        ::std::mem::size_of::<_Thrd_t>(),
        16usize,
        concat!("Size of: ", stringify!(_Thrd_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_Thrd_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_Thrd_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_Thrd_t>()))._Hnd as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_Thrd_t),
            "::",
            stringify!(_Hnd)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_Thrd_t>()))._Id as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_Thrd_t),
            "::",
            stringify!(_Id)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _VIGEM_CLIENT_T {
    pub hBusDevice: HANDLE,
}
#[test]
fn bindgen_test_layout__VIGEM_CLIENT_T() {
    assert_eq!(
        ::std::mem::size_of::<_VIGEM_CLIENT_T>(),
        8usize,
        concat!("Size of: ", stringify!(_VIGEM_CLIENT_T))
    );
    assert_eq!(
        ::std::mem::align_of::<_VIGEM_CLIENT_T>(),
        8usize,
        concat!("Alignment of ", stringify!(_VIGEM_CLIENT_T))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_VIGEM_CLIENT_T>())).hBusDevice as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_VIGEM_CLIENT_T),
            "::",
            stringify!(hBusDevice)
        )
    );
}
pub const _VIGEM_TARGET_STATE_VIGEM_TARGET_NEW: _VIGEM_TARGET_STATE = 0;
pub const _VIGEM_TARGET_STATE_VIGEM_TARGET_INITIALIZED: _VIGEM_TARGET_STATE = 1;
pub const _VIGEM_TARGET_STATE_VIGEM_TARGET_CONNECTED: _VIGEM_TARGET_STATE = 2;
pub const _VIGEM_TARGET_STATE_VIGEM_TARGET_DISCONNECTED: _VIGEM_TARGET_STATE = 3;
pub type _VIGEM_TARGET_STATE = i32;
pub use self::_VIGEM_TARGET_STATE as VIGEM_TARGET_STATE;
#[repr(C)]
#[derive(Debug)]
pub struct _VIGEM_TARGET_T {
    pub Size: ULONG,
    pub SerialNo: ULONG,
    pub State: VIGEM_TARGET_STATE,
    pub VendorId: USHORT,
    pub ProductId: USHORT,
    pub Type: VIGEM_TARGET_TYPE,
    pub Notification: FARPROC,
    pub NotificationUserData: LPVOID,
    pub closingNotificationThreads: bool,
    pub cancelNotificationThreadEvent: HANDLE,
    pub notificationThreadList: std_unique_ptr,
}
#[test]
fn bindgen_test_layout__VIGEM_TARGET_T() {
    assert_eq!(
        ::std::mem::size_of::<_VIGEM_TARGET_T>(),
        64usize,
        concat!("Size of: ", stringify!(_VIGEM_TARGET_T))
    );
    assert_eq!(
        ::std::mem::align_of::<_VIGEM_TARGET_T>(),
        8usize,
        concat!("Alignment of ", stringify!(_VIGEM_TARGET_T))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_VIGEM_TARGET_T>())).Size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_VIGEM_TARGET_T),
            "::",
            stringify!(Size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_VIGEM_TARGET_T>())).SerialNo as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_VIGEM_TARGET_T),
            "::",
            stringify!(SerialNo)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_VIGEM_TARGET_T>())).State as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_VIGEM_TARGET_T),
            "::",
            stringify!(State)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_VIGEM_TARGET_T>())).VendorId as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(_VIGEM_TARGET_T),
            "::",
            stringify!(VendorId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_VIGEM_TARGET_T>())).ProductId as *const _ as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(_VIGEM_TARGET_T),
            "::",
            stringify!(ProductId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_VIGEM_TARGET_T>())).Type as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_VIGEM_TARGET_T),
            "::",
            stringify!(Type)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_VIGEM_TARGET_T>())).Notification as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_VIGEM_TARGET_T),
            "::",
            stringify!(Notification)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_VIGEM_TARGET_T>())).NotificationUserData as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_VIGEM_TARGET_T),
            "::",
            stringify!(NotificationUserData)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_VIGEM_TARGET_T>())).closingNotificationThreads as *const _
                as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_VIGEM_TARGET_T),
            "::",
            stringify!(closingNotificationThreads)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_VIGEM_TARGET_T>())).cancelNotificationThreadEvent as *const _
                as usize
        },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_VIGEM_TARGET_T),
            "::",
            stringify!(cancelNotificationThreadEvent)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_VIGEM_TARGET_T>())).notificationThreadList as *const _ as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_VIGEM_TARGET_T),
            "::",
            stringify!(notificationThreadList)
        )
    );
}
extern "C" {
    #[link_name = "\u{1}?vigem_internal_exception_handler@@YAJPEAU_EXCEPTION_POINTERS@@@Z"]
    pub fn vigem_internal_exception_handler(apExceptionInfo: *mut _EXCEPTION_POINTERS) -> LONG;
}
#[repr(C)]
pub struct NotificationRequestPayload__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug)]
pub struct NotificationRequestPayload {
    pub vtable_: *const NotificationRequestPayload__bindgen_vtable,
    pub lpPayloadBuffer: LPVOID,
    pub payloadBufferSize: DWORD,
    pub ioControlCode: DWORD,
}
#[test]
fn bindgen_test_layout_NotificationRequestPayload() {
    assert_eq!(
        ::std::mem::size_of::<NotificationRequestPayload>(),
        24usize,
        concat!("Size of: ", stringify!(NotificationRequestPayload))
    );
    assert_eq!(
        ::std::mem::align_of::<NotificationRequestPayload>(),
        8usize,
        concat!("Alignment of ", stringify!(NotificationRequestPayload))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<NotificationRequestPayload>())).lpPayloadBuffer as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(NotificationRequestPayload),
            "::",
            stringify!(lpPayloadBuffer)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<NotificationRequestPayload>())).payloadBufferSize as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(NotificationRequestPayload),
            "::",
            stringify!(payloadBufferSize)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<NotificationRequestPayload>())).ioControlCode as *const _
                as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(NotificationRequestPayload),
            "::",
            stringify!(ioControlCode)
        )
    );
}
extern "C" {
    #[link_name = "\u{1}?vigem_notification_thread_worker@@YAXPEAU_VIGEM_CLIENT_T@@PEAU_VIGEM_TARGET_T@@V?$unique_ptr@V?$vector@V?$unique_ptr@VNotificationRequestPayload@@U?$default_delete@VNotificationRequestPayload@@@std@@@std@@V?$allocator@V?$unique_ptr@VNotificationRequestPayload@@U?$default_delete@VNotificationRequestPayload@@@std@@@std@@@2@@std@@U?$default_delete@V?$vector@V?$unique_ptr@VNotificationRequestPayload@@U?$default_delete@VNotificationRequestPayload@@@std@@@std@@V?$allocator@V?$unique_ptr@VNotificationRequestPayload@@U?$default_delete@VNotificationRequestPayload@@@std@@@std@@@2@@std@@@2@@std@@@Z"]
    pub fn vigem_notification_thread_worker(
        client: PVIGEM_CLIENT,
        target: PVIGEM_TARGET,
        pNotificationRequestPayload: std_unique_ptr,
    );
}
#[test]
fn __bindgen_test_layout_std_unique_ptr_open0_std_vector_open1_std_thread_std_allocator_open2_std_thread_close2_close1_std_default_delete_open1_std_vector_open2_std_thread_std_allocator_open3_std_thread_close3_close2_close1_close0_instantiation(
) {
    assert_eq!(
        ::std::mem::size_of::<std_unique_ptr>(),
        8usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_unique_ptr)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<std_unique_ptr>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_unique_ptr)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_vector_open0_std_thread_std_allocator_open1_std_thread_close1_close0_instantiation(
) {
    assert_eq!(
        ::std::mem::size_of::<std_vector>(),
        24usize,
        concat!("Size of template specialization: ", stringify!(std_vector))
    );
    assert_eq!(
        ::std::mem::align_of::<std_vector>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_vector)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_allocator_open0_std_thread_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_default_delete_open0_std_vector_open1_std_thread_std_allocator_open2_std_thread_close2_close1_close0_instantiation(
) {
    assert_eq!(
        ::std::mem::size_of::<std_default_delete>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_default_delete)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<std_default_delete>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_default_delete)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_vector_open0_std_thread_std_allocator_open1_std_thread_close1_close0_instantiation_1(
) {
    assert_eq!(
        ::std::mem::size_of::<std_vector>(),
        24usize,
        concat!("Size of template specialization: ", stringify!(std_vector))
    );
    assert_eq!(
        ::std::mem::align_of::<std_vector>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_vector)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_allocator_open0_std_thread_close0_instantiation_1() {
    assert_eq!(
        ::std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_unique_ptr_open0_std_vector_open1_std_unique_ptr_open2_NotificationRequestPayload_std_default_delete_open3_NotificationRequestPayload_close3_close2_std_allocator_open2_std_unique_ptr_open3_NotificationRequestPayload_std_default_delete_open4_NotificationRequestPayload_close4_close3_close2_close1_std_default_delete_open1_std_vector_open2_std_unique_ptr_open3_NotificationRequestPayload_std_default_delete_open4_NotificationRequestPayload_close4_close3_std_allocator_open3_std_unique_ptr_open4_NotificationRequestPayload_std_default_delete_open5_NotificationRequestPayload_close5_close4_close3_close2_close1_close0_instantiation(
) {
    assert_eq!(
        ::std::mem::size_of::<std_unique_ptr>(),
        8usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_unique_ptr)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<std_unique_ptr>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_unique_ptr)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_vector_open0_std_unique_ptr_open1_NotificationRequestPayload_std_default_delete_open2_NotificationRequestPayload_close2_close1_std_allocator_open1_std_unique_ptr_open2_NotificationRequestPayload_std_default_delete_open3_NotificationRequestPayload_close3_close2_close1_close0_instantiation(
) {
    assert_eq!(
        ::std::mem::size_of::<std_vector>(),
        24usize,
        concat!("Size of template specialization: ", stringify!(std_vector))
    );
    assert_eq!(
        ::std::mem::align_of::<std_vector>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_vector)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_unique_ptr_open0_NotificationRequestPayload_std_default_delete_open1_NotificationRequestPayload_close1_close0_instantiation(
) {
    assert_eq!(
        ::std::mem::size_of::<std_unique_ptr>(),
        8usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_unique_ptr)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<std_unique_ptr>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_unique_ptr)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_default_delete_open0_NotificationRequestPayload_close0_instantiation()
{
    assert_eq!(
        ::std::mem::size_of::<std_default_delete>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_default_delete)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<std_default_delete>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_default_delete)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_allocator_open0_std_unique_ptr_open1_NotificationRequestPayload_std_default_delete_open2_NotificationRequestPayload_close2_close1_close0_instantiation(
) {
    assert_eq!(
        ::std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_unique_ptr_open0_NotificationRequestPayload_std_default_delete_open1_NotificationRequestPayload_close1_close0_instantiation_1(
) {
    assert_eq!(
        ::std::mem::size_of::<std_unique_ptr>(),
        8usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_unique_ptr)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<std_unique_ptr>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_unique_ptr)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_default_delete_open0_NotificationRequestPayload_close0_instantiation_1(
) {
    assert_eq!(
        ::std::mem::size_of::<std_default_delete>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_default_delete)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<std_default_delete>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_default_delete)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_default_delete_open0_std_vector_open1_std_unique_ptr_open2_NotificationRequestPayload_std_default_delete_open3_NotificationRequestPayload_close3_close2_std_allocator_open2_std_unique_ptr_open3_NotificationRequestPayload_std_default_delete_open4_NotificationRequestPayload_close4_close3_close2_close1_close0_instantiation(
) {
    assert_eq!(
        ::std::mem::size_of::<std_default_delete>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_default_delete)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<std_default_delete>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_default_delete)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_vector_open0_std_unique_ptr_open1_NotificationRequestPayload_std_default_delete_open2_NotificationRequestPayload_close2_close1_std_allocator_open1_std_unique_ptr_open2_NotificationRequestPayload_std_default_delete_open3_NotificationRequestPayload_close3_close2_close1_close0_instantiation_1(
) {
    assert_eq!(
        ::std::mem::size_of::<std_vector>(),
        24usize,
        concat!("Size of template specialization: ", stringify!(std_vector))
    );
    assert_eq!(
        ::std::mem::align_of::<std_vector>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_vector)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_unique_ptr_open0_NotificationRequestPayload_std_default_delete_open1_NotificationRequestPayload_close1_close0_instantiation_2(
) {
    assert_eq!(
        ::std::mem::size_of::<std_unique_ptr>(),
        8usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_unique_ptr)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<std_unique_ptr>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_unique_ptr)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_default_delete_open0_NotificationRequestPayload_close0_instantiation_2(
) {
    assert_eq!(
        ::std::mem::size_of::<std_default_delete>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_default_delete)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<std_default_delete>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_default_delete)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_allocator_open0_std_unique_ptr_open1_NotificationRequestPayload_std_default_delete_open2_NotificationRequestPayload_close2_close1_close0_instantiation_1(
) {
    assert_eq!(
        ::std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_unique_ptr_open0_NotificationRequestPayload_std_default_delete_open1_NotificationRequestPayload_close1_close0_instantiation_3(
) {
    assert_eq!(
        ::std::mem::size_of::<std_unique_ptr>(),
        8usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_unique_ptr)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<std_unique_ptr>(),
        8usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_unique_ptr)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_default_delete_open0_NotificationRequestPayload_close0_instantiation_3(
) {
    assert_eq!(
        ::std::mem::size_of::<std_default_delete>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_default_delete)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<std_default_delete>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_default_delete)
        )
    );
}
