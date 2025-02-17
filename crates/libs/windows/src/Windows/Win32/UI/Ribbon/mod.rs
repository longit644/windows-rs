windows_core::imp::define_interface!(IUIApplication, IUIApplication_Vtbl, 0xd428903c_729a_491d_910d_682a08ff2522);
impl core::ops::Deref for IUIApplication {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIApplication, windows_core::IUnknown);
impl IUIApplication {
    pub unsafe fn OnViewChanged<P0>(&self, viewid: u32, typeid: UI_VIEWTYPE, view: P0, verb: UI_VIEWVERB, ureasoncode: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).OnViewChanged)(windows_core::Interface::as_raw(self), viewid, typeid, view.param().abi(), verb, ureasoncode).ok()
    }
    pub unsafe fn OnCreateUICommand(&self, commandid: u32, typeid: UI_COMMANDTYPE) -> windows_core::Result<IUICommandHandler> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OnCreateUICommand)(windows_core::Interface::as_raw(self), commandid, typeid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn OnDestroyUICommand<P0>(&self, commandid: u32, typeid: UI_COMMANDTYPE, commandhandler: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUICommandHandler>,
    {
        (windows_core::Interface::vtable(self).OnDestroyUICommand)(windows_core::Interface::as_raw(self), commandid, typeid, commandhandler.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IUIApplication_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnViewChanged: unsafe extern "system" fn(*mut core::ffi::c_void, u32, UI_VIEWTYPE, *mut core::ffi::c_void, UI_VIEWVERB, i32) -> windows_core::HRESULT,
    pub OnCreateUICommand: unsafe extern "system" fn(*mut core::ffi::c_void, u32, UI_COMMANDTYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnDestroyUICommand: unsafe extern "system" fn(*mut core::ffi::c_void, u32, UI_COMMANDTYPE, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIApplication_Impl: Sized + windows_core::IUnknownImpl {
    fn OnViewChanged(&self, viewid: u32, typeid: UI_VIEWTYPE, view: Option<&windows_core::IUnknown>, verb: UI_VIEWVERB, ureasoncode: i32) -> windows_core::Result<()>;
    fn OnCreateUICommand(&self, commandid: u32, typeid: UI_COMMANDTYPE) -> windows_core::Result<IUICommandHandler>;
    fn OnDestroyUICommand(&self, commandid: u32, typeid: UI_COMMANDTYPE, commandhandler: Option<&IUICommandHandler>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIApplication {}
impl IUIApplication_Vtbl {
    pub const fn new<Identity: IUIApplication_Impl, const OFFSET: isize>() -> IUIApplication_Vtbl {
        unsafe extern "system" fn OnViewChanged<Identity: IUIApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewid: u32, typeid: UI_VIEWTYPE, view: *mut core::ffi::c_void, verb: UI_VIEWVERB, ureasoncode: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIApplication_Impl::OnViewChanged(this, core::mem::transmute_copy(&viewid), core::mem::transmute_copy(&typeid), windows_core::from_raw_borrowed(&view), core::mem::transmute_copy(&verb), core::mem::transmute_copy(&ureasoncode)).into()
        }
        unsafe extern "system" fn OnCreateUICommand<Identity: IUIApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandid: u32, typeid: UI_COMMANDTYPE, commandhandler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIApplication_Impl::OnCreateUICommand(this, core::mem::transmute_copy(&commandid), core::mem::transmute_copy(&typeid)) {
                Ok(ok__) => {
                    commandhandler.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDestroyUICommand<Identity: IUIApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandid: u32, typeid: UI_COMMANDTYPE, commandhandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIApplication_Impl::OnDestroyUICommand(this, core::mem::transmute_copy(&commandid), core::mem::transmute_copy(&typeid), windows_core::from_raw_borrowed(&commandhandler)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnViewChanged: OnViewChanged::<Identity, OFFSET>,
            OnCreateUICommand: OnCreateUICommand::<Identity, OFFSET>,
            OnDestroyUICommand: OnDestroyUICommand::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIApplication as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUICollection, IUICollection_Vtbl, 0xdf4f45bf_6f9d_4dd7_9d68_d8f9cd18c4db);
impl core::ops::Deref for IUICollection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUICollection, windows_core::IUnknown);
impl IUICollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetItem(&self, index: u32) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetItem)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Add<P0>(&self, item: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), item.param().abi()).ok()
    }
    pub unsafe fn Insert<P0>(&self, index: u32, item: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).Insert)(windows_core::Interface::as_raw(self), index, item.param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn Replace<P0>(&self, indexreplaced: u32, itemreplacewith: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).Replace)(windows_core::Interface::as_raw(self), indexreplaced, itemreplacewith.param().abi()).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IUICollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetItem: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Insert: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Replace: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUICollection_Impl: Sized + windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetItem(&self, index: u32) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, item: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Insert(&self, index: u32, item: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
    fn Replace(&self, indexreplaced: u32, itemreplacewith: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUICollection {}
impl IUICollection_Vtbl {
    pub const fn new<Identity: IUICollection_Impl, const OFFSET: isize>() -> IUICollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: IUICollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUICollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItem<Identity: IUICollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, item: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUICollection_Impl::GetItem(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    item.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: IUICollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUICollection_Impl::Add(this, windows_core::from_raw_borrowed(&item)).into()
        }
        unsafe extern "system" fn Insert<Identity: IUICollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, item: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUICollection_Impl::Insert(this, core::mem::transmute_copy(&index), windows_core::from_raw_borrowed(&item)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: IUICollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUICollection_Impl::RemoveAt(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Replace<Identity: IUICollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexreplaced: u32, itemreplacewith: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUICollection_Impl::Replace(this, core::mem::transmute_copy(&indexreplaced), windows_core::from_raw_borrowed(&itemreplacewith)).into()
        }
        unsafe extern "system" fn Clear<Identity: IUICollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUICollection_Impl::Clear(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetItem: GetItem::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Insert: Insert::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
            Replace: Replace::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUICollection as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUICollectionChangedEvent, IUICollectionChangedEvent_Vtbl, 0x6502ae91_a14d_44b5_bbd0_62aacc581d52);
impl core::ops::Deref for IUICollectionChangedEvent {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUICollectionChangedEvent, windows_core::IUnknown);
impl IUICollectionChangedEvent {
    pub unsafe fn OnChanged<P0, P1>(&self, action: UI_COLLECTIONCHANGE, oldindex: u32, olditem: P0, newindex: u32, newitem: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).OnChanged)(windows_core::Interface::as_raw(self), action, oldindex, olditem.param().abi(), newindex, newitem.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IUICollectionChangedEvent_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnChanged: unsafe extern "system" fn(*mut core::ffi::c_void, UI_COLLECTIONCHANGE, u32, *mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUICollectionChangedEvent_Impl: Sized + windows_core::IUnknownImpl {
    fn OnChanged(&self, action: UI_COLLECTIONCHANGE, oldindex: u32, olditem: Option<&windows_core::IUnknown>, newindex: u32, newitem: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUICollectionChangedEvent {}
impl IUICollectionChangedEvent_Vtbl {
    pub const fn new<Identity: IUICollectionChangedEvent_Impl, const OFFSET: isize>() -> IUICollectionChangedEvent_Vtbl {
        unsafe extern "system" fn OnChanged<Identity: IUICollectionChangedEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, action: UI_COLLECTIONCHANGE, oldindex: u32, olditem: *mut core::ffi::c_void, newindex: u32, newitem: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUICollectionChangedEvent_Impl::OnChanged(this, core::mem::transmute_copy(&action), core::mem::transmute_copy(&oldindex), windows_core::from_raw_borrowed(&olditem), core::mem::transmute_copy(&newindex), windows_core::from_raw_borrowed(&newitem)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnChanged: OnChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUICollectionChangedEvent as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUICommandHandler, IUICommandHandler_Vtbl, 0x75ae0a2d_dc03_4c9f_8883_069660d0beb6);
impl core::ops::Deref for IUICommandHandler {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUICommandHandler, windows_core::IUnknown);
impl IUICommandHandler {
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn Execute<P0>(&self, commandid: u32, verb: UI_EXECUTIONVERB, key: Option<*const super::Shell::PropertiesSystem::PROPERTYKEY>, currentvalue: Option<*const super::super::System::Com::StructuredStorage::PROPVARIANT>, commandexecutionproperties: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUISimplePropertySet>,
    {
        (windows_core::Interface::vtable(self).Execute)(windows_core::Interface::as_raw(self), commandid, verb, core::mem::transmute(key.unwrap_or(core::ptr::null())), core::mem::transmute(currentvalue.unwrap_or(core::ptr::null())), commandexecutionproperties.param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn UpdateProperty(&self, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: Option<*const super::super::System::Com::StructuredStorage::PROPVARIANT>) -> windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UpdateProperty)(windows_core::Interface::as_raw(self), commandid, key, core::mem::transmute(currentvalue.unwrap_or(core::ptr::null())), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUICommandHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub Execute: unsafe extern "system" fn(*mut core::ffi::c_void, u32, UI_EXECUTIONVERB, *const super::Shell::PropertiesSystem::PROPERTYKEY, *const core::mem::MaybeUninit<super::super::System::Com::StructuredStorage::PROPVARIANT>, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem")))]
    Execute: usize,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub UpdateProperty: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::Shell::PropertiesSystem::PROPERTYKEY, *const core::mem::MaybeUninit<super::super::System::Com::StructuredStorage::PROPVARIANT>, *mut core::mem::MaybeUninit<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem")))]
    UpdateProperty: usize,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IUICommandHandler_Impl: Sized + windows_core::IUnknownImpl {
    fn Execute(&self, commandid: u32, verb: UI_EXECUTIONVERB, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT, commandexecutionproperties: Option<&IUISimplePropertySet>) -> windows_core::Result<()>;
    fn UpdateProperty(&self, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl windows_core::RuntimeName for IUICommandHandler {}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IUICommandHandler_Vtbl {
    pub const fn new<Identity: IUICommandHandler_Impl, const OFFSET: isize>() -> IUICommandHandler_Vtbl {
        unsafe extern "system" fn Execute<Identity: IUICommandHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandid: u32, verb: UI_EXECUTIONVERB, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: *const core::mem::MaybeUninit<super::super::System::Com::StructuredStorage::PROPVARIANT>, commandexecutionproperties: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUICommandHandler_Impl::Execute(this, core::mem::transmute_copy(&commandid), core::mem::transmute_copy(&verb), core::mem::transmute_copy(&key), core::mem::transmute_copy(&currentvalue), windows_core::from_raw_borrowed(&commandexecutionproperties)).into()
        }
        unsafe extern "system" fn UpdateProperty<Identity: IUICommandHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: *const core::mem::MaybeUninit<super::super::System::Com::StructuredStorage::PROPVARIANT>, newvalue: *mut core::mem::MaybeUninit<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUICommandHandler_Impl::UpdateProperty(this, core::mem::transmute_copy(&commandid), core::mem::transmute_copy(&key), core::mem::transmute_copy(&currentvalue)) {
                Ok(ok__) => {
                    newvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Execute: Execute::<Identity, OFFSET>,
            UpdateProperty: UpdateProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUICommandHandler as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIContextualUI, IUIContextualUI_Vtbl, 0xeea11f37_7c46_437c_8e55_b52122b29293);
impl core::ops::Deref for IUIContextualUI {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIContextualUI, windows_core::IUnknown);
impl IUIContextualUI {
    pub unsafe fn ShowAtLocation(&self, x: i32, y: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ShowAtLocation)(windows_core::Interface::as_raw(self), x, y).ok()
    }
}
#[repr(C)]
pub struct IUIContextualUI_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ShowAtLocation: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
}
pub trait IUIContextualUI_Impl: Sized + windows_core::IUnknownImpl {
    fn ShowAtLocation(&self, x: i32, y: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIContextualUI {}
impl IUIContextualUI_Vtbl {
    pub const fn new<Identity: IUIContextualUI_Impl, const OFFSET: isize>() -> IUIContextualUI_Vtbl {
        unsafe extern "system" fn ShowAtLocation<Identity: IUIContextualUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIContextualUI_Impl::ShowAtLocation(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ShowAtLocation: ShowAtLocation::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIContextualUI as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIEventLogger, IUIEventLogger_Vtbl, 0xec3e1034_dbf4_41a1_95d5_03e0f1026e05);
impl core::ops::Deref for IUIEventLogger {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIEventLogger, windows_core::IUnknown);
impl IUIEventLogger {
    pub unsafe fn OnUIEvent(&self, peventparams: *const UI_EVENTPARAMS) {
        (windows_core::Interface::vtable(self).OnUIEvent)(windows_core::Interface::as_raw(self), peventparams)
    }
}
#[repr(C)]
pub struct IUIEventLogger_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnUIEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *const UI_EVENTPARAMS),
}
pub trait IUIEventLogger_Impl: Sized + windows_core::IUnknownImpl {
    fn OnUIEvent(&self, peventparams: *const UI_EVENTPARAMS);
}
impl windows_core::RuntimeName for IUIEventLogger {}
impl IUIEventLogger_Vtbl {
    pub const fn new<Identity: IUIEventLogger_Impl, const OFFSET: isize>() -> IUIEventLogger_Vtbl {
        unsafe extern "system" fn OnUIEvent<Identity: IUIEventLogger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventparams: *const UI_EVENTPARAMS) {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIEventLogger_Impl::OnUIEvent(this, core::mem::transmute_copy(&peventparams))
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnUIEvent: OnUIEvent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIEventLogger as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIEventingManager, IUIEventingManager_Vtbl, 0x3be6ea7f_9a9b_4198_9368_9b0f923bd534);
impl core::ops::Deref for IUIEventingManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIEventingManager, windows_core::IUnknown);
impl IUIEventingManager {
    pub unsafe fn SetEventLogger<P0>(&self, eventlogger: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIEventLogger>,
    {
        (windows_core::Interface::vtable(self).SetEventLogger)(windows_core::Interface::as_raw(self), eventlogger.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IUIEventingManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetEventLogger: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIEventingManager_Impl: Sized + windows_core::IUnknownImpl {
    fn SetEventLogger(&self, eventlogger: Option<&IUIEventLogger>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIEventingManager {}
impl IUIEventingManager_Vtbl {
    pub const fn new<Identity: IUIEventingManager_Impl, const OFFSET: isize>() -> IUIEventingManager_Vtbl {
        unsafe extern "system" fn SetEventLogger<Identity: IUIEventingManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventlogger: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIEventingManager_Impl::SetEventLogger(this, windows_core::from_raw_borrowed(&eventlogger)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetEventLogger: SetEventLogger::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIEventingManager as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIFramework, IUIFramework_Vtbl, 0xf4f0385d_6872_43a8_ad09_4c339cb3f5c5);
impl core::ops::Deref for IUIFramework {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIFramework, windows_core::IUnknown);
impl IUIFramework {
    pub unsafe fn Initialize<P0, P1>(&self, framewnd: P0, application: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        P1: windows_core::Param<IUIApplication>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), framewnd.param().abi(), application.param().abi()).ok()
    }
    pub unsafe fn Destroy(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Destroy)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn LoadUI<P0, P1>(&self, instance: P0, resourcename: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).LoadUI)(windows_core::Interface::as_raw(self), instance.param().abi(), resourcename.param().abi()).ok()
    }
    pub unsafe fn GetView(&self, viewid: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetView)(windows_core::Interface::as_raw(self), viewid, riid, ppv).ok()
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetUICommandProperty(&self, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUICommandProperty)(windows_core::Interface::as_raw(self), commandid, key, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn SetUICommandProperty(&self, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetUICommandProperty)(windows_core::Interface::as_raw(self), commandid, key, core::mem::transmute(value)).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn InvalidateUICommand(&self, commandid: u32, flags: UI_INVALIDATIONS, key: Option<*const super::Shell::PropertiesSystem::PROPERTYKEY>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InvalidateUICommand)(windows_core::Interface::as_raw(self), commandid, flags, core::mem::transmute(key.unwrap_or(core::ptr::null()))).ok()
    }
    pub unsafe fn FlushPendingInvalidations(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FlushPendingInvalidations)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetModes(&self, imodes: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetModes)(windows_core::Interface::as_raw(self), imodes).ok()
    }
}
#[repr(C)]
pub struct IUIFramework_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Destroy: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LoadUI: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HINSTANCE, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetView: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub GetUICommandProperty: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::Shell::PropertiesSystem::PROPERTYKEY, *mut core::mem::MaybeUninit<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem")))]
    GetUICommandProperty: usize,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub SetUICommandProperty: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::Shell::PropertiesSystem::PROPERTYKEY, *const core::mem::MaybeUninit<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem")))]
    SetUICommandProperty: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub InvalidateUICommand: unsafe extern "system" fn(*mut core::ffi::c_void, u32, UI_INVALIDATIONS, *const super::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    InvalidateUICommand: usize,
    pub FlushPendingInvalidations: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetModes: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IUIFramework_Impl: Sized + windows_core::IUnknownImpl {
    fn Initialize(&self, framewnd: super::super::Foundation::HWND, application: Option<&IUIApplication>) -> windows_core::Result<()>;
    fn Destroy(&self) -> windows_core::Result<()>;
    fn LoadUI(&self, instance: super::super::Foundation::HINSTANCE, resourcename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetView(&self, viewid: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetUICommandProperty(&self, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn SetUICommandProperty(&self, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()>;
    fn InvalidateUICommand(&self, commandid: u32, flags: UI_INVALIDATIONS, key: *const super::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<()>;
    fn FlushPendingInvalidations(&self) -> windows_core::Result<()>;
    fn SetModes(&self, imodes: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl windows_core::RuntimeName for IUIFramework {}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IUIFramework_Vtbl {
    pub const fn new<Identity: IUIFramework_Impl, const OFFSET: isize>() -> IUIFramework_Vtbl {
        unsafe extern "system" fn Initialize<Identity: IUIFramework_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, framewnd: super::super::Foundation::HWND, application: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIFramework_Impl::Initialize(this, core::mem::transmute_copy(&framewnd), windows_core::from_raw_borrowed(&application)).into()
        }
        unsafe extern "system" fn Destroy<Identity: IUIFramework_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIFramework_Impl::Destroy(this).into()
        }
        unsafe extern "system" fn LoadUI<Identity: IUIFramework_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, instance: super::super::Foundation::HINSTANCE, resourcename: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIFramework_Impl::LoadUI(this, core::mem::transmute_copy(&instance), core::mem::transmute(&resourcename)).into()
        }
        unsafe extern "system" fn GetView<Identity: IUIFramework_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewid: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIFramework_Impl::GetView(this, core::mem::transmute_copy(&viewid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetUICommandProperty<Identity: IUIFramework_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *mut core::mem::MaybeUninit<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIFramework_Impl::GetUICommandProperty(this, core::mem::transmute_copy(&commandid), core::mem::transmute_copy(&key)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUICommandProperty<Identity: IUIFramework_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *const core::mem::MaybeUninit<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIFramework_Impl::SetUICommandProperty(this, core::mem::transmute_copy(&commandid), core::mem::transmute_copy(&key), core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn InvalidateUICommand<Identity: IUIFramework_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandid: u32, flags: UI_INVALIDATIONS, key: *const super::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIFramework_Impl::InvalidateUICommand(this, core::mem::transmute_copy(&commandid), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&key)).into()
        }
        unsafe extern "system" fn FlushPendingInvalidations<Identity: IUIFramework_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIFramework_Impl::FlushPendingInvalidations(this).into()
        }
        unsafe extern "system" fn SetModes<Identity: IUIFramework_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imodes: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIFramework_Impl::SetModes(this, core::mem::transmute_copy(&imodes)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Destroy: Destroy::<Identity, OFFSET>,
            LoadUI: LoadUI::<Identity, OFFSET>,
            GetView: GetView::<Identity, OFFSET>,
            GetUICommandProperty: GetUICommandProperty::<Identity, OFFSET>,
            SetUICommandProperty: SetUICommandProperty::<Identity, OFFSET>,
            InvalidateUICommand: InvalidateUICommand::<Identity, OFFSET>,
            FlushPendingInvalidations: FlushPendingInvalidations::<Identity, OFFSET>,
            SetModes: SetModes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIFramework as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIImage, IUIImage_Vtbl, 0x23c8c838_4de6_436b_ab01_5554bb7c30dd);
impl core::ops::Deref for IUIImage {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIImage, windows_core::IUnknown);
impl IUIImage {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetBitmap(&self) -> windows_core::Result<super::super::Graphics::Gdi::HBITMAP> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBitmap)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IUIImage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::Gdi::HBITMAP) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetBitmap: usize,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IUIImage_Impl: Sized + windows_core::IUnknownImpl {
    fn GetBitmap(&self) -> windows_core::Result<super::super::Graphics::Gdi::HBITMAP>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IUIImage {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IUIImage_Vtbl {
    pub const fn new<Identity: IUIImage_Impl, const OFFSET: isize>() -> IUIImage_Vtbl {
        unsafe extern "system" fn GetBitmap<Identity: IUIImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmap: *mut super::super::Graphics::Gdi::HBITMAP) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIImage_Impl::GetBitmap(this) {
                Ok(ok__) => {
                    bitmap.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetBitmap: GetBitmap::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIImage as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIImageFromBitmap, IUIImageFromBitmap_Vtbl, 0x18aba7f3_4c1c_4ba2_bf6c_f5c3326fa816);
impl core::ops::Deref for IUIImageFromBitmap {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIImageFromBitmap, windows_core::IUnknown);
impl IUIImageFromBitmap {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateImage<P0>(&self, bitmap: P0, options: UI_OWNERSHIP) -> windows_core::Result<IUIImage>
    where
        P0: windows_core::Param<super::super::Graphics::Gdi::HBITMAP>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateImage)(windows_core::Interface::as_raw(self), bitmap.param().abi(), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUIImageFromBitmap_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub CreateImage: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Graphics::Gdi::HBITMAP, UI_OWNERSHIP, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    CreateImage: usize,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IUIImageFromBitmap_Impl: Sized + windows_core::IUnknownImpl {
    fn CreateImage(&self, bitmap: super::super::Graphics::Gdi::HBITMAP, options: UI_OWNERSHIP) -> windows_core::Result<IUIImage>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IUIImageFromBitmap {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IUIImageFromBitmap_Vtbl {
    pub const fn new<Identity: IUIImageFromBitmap_Impl, const OFFSET: isize>() -> IUIImageFromBitmap_Vtbl {
        unsafe extern "system" fn CreateImage<Identity: IUIImageFromBitmap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmap: super::super::Graphics::Gdi::HBITMAP, options: UI_OWNERSHIP, image: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIImageFromBitmap_Impl::CreateImage(this, core::mem::transmute_copy(&bitmap), core::mem::transmute_copy(&options)) {
                Ok(ok__) => {
                    image.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateImage: CreateImage::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIImageFromBitmap as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIRibbon, IUIRibbon_Vtbl, 0x803982ab_370a_4f7e_a9e7_8784036a6e26);
impl core::ops::Deref for IUIRibbon {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIRibbon, windows_core::IUnknown);
impl IUIRibbon {
    pub unsafe fn GetHeight(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHeight)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LoadSettingsFromStream<P0>(&self, pstream: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).LoadSettingsFromStream)(windows_core::Interface::as_raw(self), pstream.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SaveSettingsToStream<P0>(&self, pstream: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).SaveSettingsToStream)(windows_core::Interface::as_raw(self), pstream.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IUIRibbon_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub LoadSettingsFromStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LoadSettingsFromStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SaveSettingsToStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SaveSettingsToStream: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUIRibbon_Impl: Sized + windows_core::IUnknownImpl {
    fn GetHeight(&self) -> windows_core::Result<u32>;
    fn LoadSettingsFromStream(&self, pstream: Option<&super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn SaveSettingsToStream(&self, pstream: Option<&super::super::System::Com::IStream>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUIRibbon {}
#[cfg(feature = "Win32_System_Com")]
impl IUIRibbon_Vtbl {
    pub const fn new<Identity: IUIRibbon_Impl, const OFFSET: isize>() -> IUIRibbon_Vtbl {
        unsafe extern "system" fn GetHeight<Identity: IUIRibbon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cy: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIRibbon_Impl::GetHeight(this) {
                Ok(ok__) => {
                    cy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadSettingsFromStream<Identity: IUIRibbon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIRibbon_Impl::LoadSettingsFromStream(this, windows_core::from_raw_borrowed(&pstream)).into()
        }
        unsafe extern "system" fn SaveSettingsToStream<Identity: IUIRibbon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIRibbon_Impl::SaveSettingsToStream(this, windows_core::from_raw_borrowed(&pstream)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetHeight: GetHeight::<Identity, OFFSET>,
            LoadSettingsFromStream: LoadSettingsFromStream::<Identity, OFFSET>,
            SaveSettingsToStream: SaveSettingsToStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIRibbon as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUISimplePropertySet, IUISimplePropertySet_Vtbl, 0xc205bb48_5b1c_4219_a106_15bd0a5f24e2);
impl core::ops::Deref for IUISimplePropertySet {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUISimplePropertySet, windows_core::IUnknown);
impl IUISimplePropertySet {
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetValue(&self, key: *const super::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), key, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUISimplePropertySet_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Shell::PropertiesSystem::PROPERTYKEY, *mut core::mem::MaybeUninit<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem")))]
    GetValue: usize,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IUISimplePropertySet_Impl: Sized + windows_core::IUnknownImpl {
    fn GetValue(&self, key: *const super::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl windows_core::RuntimeName for IUISimplePropertySet {}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IUISimplePropertySet_Vtbl {
    pub const fn new<Identity: IUISimplePropertySet_Impl, const OFFSET: isize>() -> IUISimplePropertySet_Vtbl {
        unsafe extern "system" fn GetValue<Identity: IUISimplePropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *mut core::mem::MaybeUninit<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUISimplePropertySet_Impl::GetValue(this, core::mem::transmute_copy(&key)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetValue: GetValue::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUISimplePropertySet as windows_core::Interface>::IID
    }
}
pub const LIBID_UIRibbon: windows_core::GUID = windows_core::GUID::from_u128(0x942f35c2_e83b_45ef_b085_ac295dd63d5b);
pub const UI_ALL_COMMANDS: u32 = 0u32;
pub const UI_COLLECTIONCHANGE_INSERT: UI_COLLECTIONCHANGE = UI_COLLECTIONCHANGE(0i32);
pub const UI_COLLECTIONCHANGE_REMOVE: UI_COLLECTIONCHANGE = UI_COLLECTIONCHANGE(1i32);
pub const UI_COLLECTIONCHANGE_REPLACE: UI_COLLECTIONCHANGE = UI_COLLECTIONCHANGE(2i32);
pub const UI_COLLECTIONCHANGE_RESET: UI_COLLECTIONCHANGE = UI_COLLECTIONCHANGE(3i32);
pub const UI_COLLECTION_INVALIDINDEX: u32 = 4294967295u32;
pub const UI_COMMANDTYPE_ACTION: UI_COMMANDTYPE = UI_COMMANDTYPE(2i32);
pub const UI_COMMANDTYPE_ANCHOR: UI_COMMANDTYPE = UI_COMMANDTYPE(3i32);
pub const UI_COMMANDTYPE_BOOLEAN: UI_COMMANDTYPE = UI_COMMANDTYPE(8i32);
pub const UI_COMMANDTYPE_COLLECTION: UI_COMMANDTYPE = UI_COMMANDTYPE(5i32);
pub const UI_COMMANDTYPE_COLORANCHOR: UI_COMMANDTYPE = UI_COMMANDTYPE(11i32);
pub const UI_COMMANDTYPE_COLORCOLLECTION: UI_COMMANDTYPE = UI_COMMANDTYPE(12i32);
pub const UI_COMMANDTYPE_COMMANDCOLLECTION: UI_COMMANDTYPE = UI_COMMANDTYPE(6i32);
pub const UI_COMMANDTYPE_CONTEXT: UI_COMMANDTYPE = UI_COMMANDTYPE(4i32);
pub const UI_COMMANDTYPE_DECIMAL: UI_COMMANDTYPE = UI_COMMANDTYPE(7i32);
pub const UI_COMMANDTYPE_FONT: UI_COMMANDTYPE = UI_COMMANDTYPE(9i32);
pub const UI_COMMANDTYPE_GROUP: UI_COMMANDTYPE = UI_COMMANDTYPE(1i32);
pub const UI_COMMANDTYPE_RECENTITEMS: UI_COMMANDTYPE = UI_COMMANDTYPE(10i32);
pub const UI_COMMANDTYPE_UNKNOWN: UI_COMMANDTYPE = UI_COMMANDTYPE(0i32);
pub const UI_CONTEXTAVAILABILITY_ACTIVE: UI_CONTEXTAVAILABILITY = UI_CONTEXTAVAILABILITY(2i32);
pub const UI_CONTEXTAVAILABILITY_AVAILABLE: UI_CONTEXTAVAILABILITY = UI_CONTEXTAVAILABILITY(1i32);
pub const UI_CONTEXTAVAILABILITY_NOTAVAILABLE: UI_CONTEXTAVAILABILITY = UI_CONTEXTAVAILABILITY(0i32);
pub const UI_CONTROLDOCK_BOTTOM: UI_CONTROLDOCK = UI_CONTROLDOCK(3i32);
pub const UI_CONTROLDOCK_TOP: UI_CONTROLDOCK = UI_CONTROLDOCK(1i32);
pub const UI_EVENTLOCATION_ApplicationMenu: UI_EVENTLOCATION = UI_EVENTLOCATION(2i32);
pub const UI_EVENTLOCATION_ContextPopup: UI_EVENTLOCATION = UI_EVENTLOCATION(3i32);
pub const UI_EVENTLOCATION_QAT: UI_EVENTLOCATION = UI_EVENTLOCATION(1i32);
pub const UI_EVENTLOCATION_Ribbon: UI_EVENTLOCATION = UI_EVENTLOCATION(0i32);
pub const UI_EVENTTYPE_ApplicationMenuOpened: UI_EVENTTYPE = UI_EVENTTYPE(0i32);
pub const UI_EVENTTYPE_ApplicationModeSwitched: UI_EVENTTYPE = UI_EVENTTYPE(3i32);
pub const UI_EVENTTYPE_CommandExecuted: UI_EVENTTYPE = UI_EVENTTYPE(6i32);
pub const UI_EVENTTYPE_MenuOpened: UI_EVENTTYPE = UI_EVENTTYPE(5i32);
pub const UI_EVENTTYPE_RibbonExpanded: UI_EVENTTYPE = UI_EVENTTYPE(2i32);
pub const UI_EVENTTYPE_RibbonMinimized: UI_EVENTTYPE = UI_EVENTTYPE(1i32);
pub const UI_EVENTTYPE_TabActivated: UI_EVENTTYPE = UI_EVENTTYPE(4i32);
pub const UI_EVENTTYPE_TooltipShown: UI_EVENTTYPE = UI_EVENTTYPE(7i32);
pub const UI_EXECUTIONVERB_CANCELPREVIEW: UI_EXECUTIONVERB = UI_EXECUTIONVERB(2i32);
pub const UI_EXECUTIONVERB_EXECUTE: UI_EXECUTIONVERB = UI_EXECUTIONVERB(0i32);
pub const UI_EXECUTIONVERB_PREVIEW: UI_EXECUTIONVERB = UI_EXECUTIONVERB(1i32);
pub const UI_FONTDELTASIZE_GROW: UI_FONTDELTASIZE = UI_FONTDELTASIZE(0i32);
pub const UI_FONTDELTASIZE_SHRINK: UI_FONTDELTASIZE = UI_FONTDELTASIZE(1i32);
pub const UI_FONTPROPERTIES_NOTAVAILABLE: UI_FONTPROPERTIES = UI_FONTPROPERTIES(0i32);
pub const UI_FONTPROPERTIES_NOTSET: UI_FONTPROPERTIES = UI_FONTPROPERTIES(1i32);
pub const UI_FONTPROPERTIES_SET: UI_FONTPROPERTIES = UI_FONTPROPERTIES(2i32);
pub const UI_FONTUNDERLINE_NOTAVAILABLE: UI_FONTUNDERLINE = UI_FONTUNDERLINE(0i32);
pub const UI_FONTUNDERLINE_NOTSET: UI_FONTUNDERLINE = UI_FONTUNDERLINE(1i32);
pub const UI_FONTUNDERLINE_SET: UI_FONTUNDERLINE = UI_FONTUNDERLINE(2i32);
pub const UI_FONTVERTICALPOSITION_NOTAVAILABLE: UI_FONTVERTICALPOSITION = UI_FONTVERTICALPOSITION(0i32);
pub const UI_FONTVERTICALPOSITION_NOTSET: UI_FONTVERTICALPOSITION = UI_FONTVERTICALPOSITION(1i32);
pub const UI_FONTVERTICALPOSITION_SUBSCRIPT: UI_FONTVERTICALPOSITION = UI_FONTVERTICALPOSITION(3i32);
pub const UI_FONTVERTICALPOSITION_SUPERSCRIPT: UI_FONTVERTICALPOSITION = UI_FONTVERTICALPOSITION(2i32);
pub const UI_INVALIDATIONS_ALLPROPERTIES: UI_INVALIDATIONS = UI_INVALIDATIONS(8i32);
pub const UI_INVALIDATIONS_PROPERTY: UI_INVALIDATIONS = UI_INVALIDATIONS(4i32);
pub const UI_INVALIDATIONS_STATE: UI_INVALIDATIONS = UI_INVALIDATIONS(1i32);
pub const UI_INVALIDATIONS_VALUE: UI_INVALIDATIONS = UI_INVALIDATIONS(2i32);
pub const UI_OWNERSHIP_COPY: UI_OWNERSHIP = UI_OWNERSHIP(1i32);
pub const UI_OWNERSHIP_TRANSFER: UI_OWNERSHIP = UI_OWNERSHIP(0i32);
pub const UI_SWATCHCOLORMODE_MONOCHROME: UI_SWATCHCOLORMODE = UI_SWATCHCOLORMODE(1i32);
pub const UI_SWATCHCOLORMODE_NORMAL: UI_SWATCHCOLORMODE = UI_SWATCHCOLORMODE(0i32);
pub const UI_SWATCHCOLORTYPE_AUTOMATIC: UI_SWATCHCOLORTYPE = UI_SWATCHCOLORTYPE(1i32);
pub const UI_SWATCHCOLORTYPE_NOCOLOR: UI_SWATCHCOLORTYPE = UI_SWATCHCOLORTYPE(0i32);
pub const UI_SWATCHCOLORTYPE_RGB: UI_SWATCHCOLORTYPE = UI_SWATCHCOLORTYPE(2i32);
pub const UI_VIEWTYPE_RIBBON: UI_VIEWTYPE = UI_VIEWTYPE(1i32);
pub const UI_VIEWVERB_CREATE: UI_VIEWVERB = UI_VIEWVERB(0i32);
pub const UI_VIEWVERB_DESTROY: UI_VIEWVERB = UI_VIEWVERB(1i32);
pub const UI_VIEWVERB_ERROR: UI_VIEWVERB = UI_VIEWVERB(3i32);
pub const UI_VIEWVERB_SIZE: UI_VIEWVERB = UI_VIEWVERB(2i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UI_COLLECTIONCHANGE(pub i32);
impl windows_core::TypeKind for UI_COLLECTIONCHANGE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UI_COLLECTIONCHANGE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UI_COLLECTIONCHANGE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UI_COMMANDTYPE(pub i32);
impl windows_core::TypeKind for UI_COMMANDTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UI_COMMANDTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UI_COMMANDTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UI_CONTEXTAVAILABILITY(pub i32);
impl windows_core::TypeKind for UI_CONTEXTAVAILABILITY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UI_CONTEXTAVAILABILITY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UI_CONTEXTAVAILABILITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UI_CONTROLDOCK(pub i32);
impl windows_core::TypeKind for UI_CONTROLDOCK {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UI_CONTROLDOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UI_CONTROLDOCK").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UI_EVENTLOCATION(pub i32);
impl windows_core::TypeKind for UI_EVENTLOCATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UI_EVENTLOCATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UI_EVENTLOCATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UI_EVENTTYPE(pub i32);
impl windows_core::TypeKind for UI_EVENTTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UI_EVENTTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UI_EVENTTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UI_EXECUTIONVERB(pub i32);
impl windows_core::TypeKind for UI_EXECUTIONVERB {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UI_EXECUTIONVERB {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UI_EXECUTIONVERB").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UI_FONTDELTASIZE(pub i32);
impl windows_core::TypeKind for UI_FONTDELTASIZE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UI_FONTDELTASIZE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UI_FONTDELTASIZE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UI_FONTPROPERTIES(pub i32);
impl windows_core::TypeKind for UI_FONTPROPERTIES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UI_FONTPROPERTIES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UI_FONTPROPERTIES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UI_FONTUNDERLINE(pub i32);
impl windows_core::TypeKind for UI_FONTUNDERLINE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UI_FONTUNDERLINE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UI_FONTUNDERLINE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UI_FONTVERTICALPOSITION(pub i32);
impl windows_core::TypeKind for UI_FONTVERTICALPOSITION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UI_FONTVERTICALPOSITION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UI_FONTVERTICALPOSITION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UI_INVALIDATIONS(pub i32);
impl windows_core::TypeKind for UI_INVALIDATIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UI_INVALIDATIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UI_INVALIDATIONS").field(&self.0).finish()
    }
}
impl UI_INVALIDATIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for UI_INVALIDATIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for UI_INVALIDATIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for UI_INVALIDATIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for UI_INVALIDATIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for UI_INVALIDATIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UI_OWNERSHIP(pub i32);
impl windows_core::TypeKind for UI_OWNERSHIP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UI_OWNERSHIP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UI_OWNERSHIP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UI_SWATCHCOLORMODE(pub i32);
impl windows_core::TypeKind for UI_SWATCHCOLORMODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UI_SWATCHCOLORMODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UI_SWATCHCOLORMODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UI_SWATCHCOLORTYPE(pub i32);
impl windows_core::TypeKind for UI_SWATCHCOLORTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UI_SWATCHCOLORTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UI_SWATCHCOLORTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UI_VIEWTYPE(pub i32);
impl windows_core::TypeKind for UI_VIEWTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UI_VIEWTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UI_VIEWTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UI_VIEWVERB(pub i32);
impl windows_core::TypeKind for UI_VIEWVERB {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UI_VIEWVERB {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UI_VIEWVERB").field(&self.0).finish()
    }
}
pub const UIRibbonFramework: windows_core::GUID = windows_core::GUID::from_u128(0x926749fa_2615_4987_8845_c33e65f2b957);
pub const UIRibbonImageFromBitmapFactory: windows_core::GUID = windows_core::GUID::from_u128(0x0f7434b6_59b6_4250_999e_d168d6ae4293);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UI_EVENTPARAMS {
    pub EventType: UI_EVENTTYPE,
    pub Anonymous: UI_EVENTPARAMS_0,
}
impl windows_core::TypeKind for UI_EVENTPARAMS {
    type TypeKind = windows_core::CopyType;
}
impl Default for UI_EVENTPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union UI_EVENTPARAMS_0 {
    pub Modes: i32,
    pub Params: UI_EVENTPARAMS_COMMAND,
}
impl windows_core::TypeKind for UI_EVENTPARAMS_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for UI_EVENTPARAMS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UI_EVENTPARAMS_COMMAND {
    pub CommandID: u32,
    pub CommandName: windows_core::PCWSTR,
    pub ParentCommandID: u32,
    pub ParentCommandName: windows_core::PCWSTR,
    pub SelectionIndex: u32,
    pub Location: UI_EVENTLOCATION,
}
impl windows_core::TypeKind for UI_EVENTPARAMS_COMMAND {
    type TypeKind = windows_core::CopyType;
}
impl Default for UI_EVENTPARAMS_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
