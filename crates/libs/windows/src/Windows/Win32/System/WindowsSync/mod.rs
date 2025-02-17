windows_core::imp::define_interface!(IAsynchronousDataRetriever, IAsynchronousDataRetriever_Vtbl, 0x9fc7e470_61ea_4a88_9be4_df56a27cfef2);
impl core::ops::Deref for IAsynchronousDataRetriever {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAsynchronousDataRetriever, windows_core::IUnknown);
impl IAsynchronousDataRetriever {
    pub unsafe fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetIdParameters)(windows_core::Interface::as_raw(self), pidparameters).ok()
    }
    pub unsafe fn RegisterCallback<P0>(&self, pdataretrievercallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDataRetrieverCallback>,
    {
        (windows_core::Interface::vtable(self).RegisterCallback)(windows_core::Interface::as_raw(self), pdataretrievercallback.param().abi()).ok()
    }
    pub unsafe fn RevokeCallback<P0>(&self, pdataretrievercallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDataRetrieverCallback>,
    {
        (windows_core::Interface::vtable(self).RevokeCallback)(windows_core::Interface::as_raw(self), pdataretrievercallback.param().abi()).ok()
    }
    pub unsafe fn LoadChangeData<P0>(&self, ploadchangecontext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ILoadChangeContext>,
    {
        (windows_core::Interface::vtable(self).LoadChangeData)(windows_core::Interface::as_raw(self), ploadchangecontext.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IAsynchronousDataRetriever_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetIdParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ID_PARAMETERS) -> windows_core::HRESULT,
    pub RegisterCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RevokeCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LoadChangeData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAsynchronousDataRetriever_Impl: Sized + windows_core::IUnknownImpl {
    fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> windows_core::Result<()>;
    fn RegisterCallback(&self, pdataretrievercallback: Option<&IDataRetrieverCallback>) -> windows_core::Result<()>;
    fn RevokeCallback(&self, pdataretrievercallback: Option<&IDataRetrieverCallback>) -> windows_core::Result<()>;
    fn LoadChangeData(&self, ploadchangecontext: Option<&ILoadChangeContext>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAsynchronousDataRetriever {}
impl IAsynchronousDataRetriever_Vtbl {
    pub const fn new<Identity: IAsynchronousDataRetriever_Impl, const OFFSET: isize>() -> IAsynchronousDataRetriever_Vtbl {
        unsafe extern "system" fn GetIdParameters<Identity: IAsynchronousDataRetriever_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAsynchronousDataRetriever_Impl::GetIdParameters(this, core::mem::transmute_copy(&pidparameters)).into()
        }
        unsafe extern "system" fn RegisterCallback<Identity: IAsynchronousDataRetriever_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataretrievercallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAsynchronousDataRetriever_Impl::RegisterCallback(this, windows_core::from_raw_borrowed(&pdataretrievercallback)).into()
        }
        unsafe extern "system" fn RevokeCallback<Identity: IAsynchronousDataRetriever_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataretrievercallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAsynchronousDataRetriever_Impl::RevokeCallback(this, windows_core::from_raw_borrowed(&pdataretrievercallback)).into()
        }
        unsafe extern "system" fn LoadChangeData<Identity: IAsynchronousDataRetriever_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ploadchangecontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAsynchronousDataRetriever_Impl::LoadChangeData(this, windows_core::from_raw_borrowed(&ploadchangecontext)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIdParameters: GetIdParameters::<Identity, OFFSET>,
            RegisterCallback: RegisterCallback::<Identity, OFFSET>,
            RevokeCallback: RevokeCallback::<Identity, OFFSET>,
            LoadChangeData: LoadChangeData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAsynchronousDataRetriever as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IChangeConflict, IChangeConflict_Vtbl, 0x014ebf97_9f20_4f7a_bdd4_25979c77c002);
impl core::ops::Deref for IChangeConflict {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IChangeConflict, windows_core::IUnknown);
impl IChangeConflict {
    pub unsafe fn GetDestinationProviderConflictingChange(&self) -> windows_core::Result<ISyncChange> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDestinationProviderConflictingChange)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSourceProviderConflictingChange(&self) -> windows_core::Result<ISyncChange> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSourceProviderConflictingChange)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDestinationProviderConflictingData(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDestinationProviderConflictingData)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSourceProviderConflictingData(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSourceProviderConflictingData)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetResolveActionForChange(&self, presolveaction: *mut SYNC_RESOLVE_ACTION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetResolveActionForChange)(windows_core::Interface::as_raw(self), presolveaction).ok()
    }
    pub unsafe fn SetResolveActionForChange(&self, resolveaction: SYNC_RESOLVE_ACTION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetResolveActionForChange)(windows_core::Interface::as_raw(self), resolveaction).ok()
    }
    pub unsafe fn GetResolveActionForChangeUnit<P0>(&self, pchangeunit: P0, presolveaction: *mut SYNC_RESOLVE_ACTION) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISyncChangeUnit>,
    {
        (windows_core::Interface::vtable(self).GetResolveActionForChangeUnit)(windows_core::Interface::as_raw(self), pchangeunit.param().abi(), presolveaction).ok()
    }
    pub unsafe fn SetResolveActionForChangeUnit<P0>(&self, pchangeunit: P0, resolveaction: SYNC_RESOLVE_ACTION) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISyncChangeUnit>,
    {
        (windows_core::Interface::vtable(self).SetResolveActionForChangeUnit)(windows_core::Interface::as_raw(self), pchangeunit.param().abi(), resolveaction).ok()
    }
}
#[repr(C)]
pub struct IChangeConflict_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDestinationProviderConflictingChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSourceProviderConflictingChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDestinationProviderConflictingData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSourceProviderConflictingData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetResolveActionForChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SYNC_RESOLVE_ACTION) -> windows_core::HRESULT,
    pub SetResolveActionForChange: unsafe extern "system" fn(*mut core::ffi::c_void, SYNC_RESOLVE_ACTION) -> windows_core::HRESULT,
    pub GetResolveActionForChangeUnit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut SYNC_RESOLVE_ACTION) -> windows_core::HRESULT,
    pub SetResolveActionForChangeUnit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, SYNC_RESOLVE_ACTION) -> windows_core::HRESULT,
}
pub trait IChangeConflict_Impl: Sized + windows_core::IUnknownImpl {
    fn GetDestinationProviderConflictingChange(&self) -> windows_core::Result<ISyncChange>;
    fn GetSourceProviderConflictingChange(&self) -> windows_core::Result<ISyncChange>;
    fn GetDestinationProviderConflictingData(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetSourceProviderConflictingData(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetResolveActionForChange(&self, presolveaction: *mut SYNC_RESOLVE_ACTION) -> windows_core::Result<()>;
    fn SetResolveActionForChange(&self, resolveaction: SYNC_RESOLVE_ACTION) -> windows_core::Result<()>;
    fn GetResolveActionForChangeUnit(&self, pchangeunit: Option<&ISyncChangeUnit>, presolveaction: *mut SYNC_RESOLVE_ACTION) -> windows_core::Result<()>;
    fn SetResolveActionForChangeUnit(&self, pchangeunit: Option<&ISyncChangeUnit>, resolveaction: SYNC_RESOLVE_ACTION) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IChangeConflict {}
impl IChangeConflict_Vtbl {
    pub const fn new<Identity: IChangeConflict_Impl, const OFFSET: isize>() -> IChangeConflict_Vtbl {
        unsafe extern "system" fn GetDestinationProviderConflictingChange<Identity: IChangeConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppconflictingchange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IChangeConflict_Impl::GetDestinationProviderConflictingChange(this) {
                Ok(ok__) => {
                    ppconflictingchange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceProviderConflictingChange<Identity: IChangeConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppconflictingchange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IChangeConflict_Impl::GetSourceProviderConflictingChange(this) {
                Ok(ok__) => {
                    ppconflictingchange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestinationProviderConflictingData<Identity: IChangeConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppconflictingdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IChangeConflict_Impl::GetDestinationProviderConflictingData(this) {
                Ok(ok__) => {
                    ppconflictingdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceProviderConflictingData<Identity: IChangeConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppconflictingdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IChangeConflict_Impl::GetSourceProviderConflictingData(this) {
                Ok(ok__) => {
                    ppconflictingdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResolveActionForChange<Identity: IChangeConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presolveaction: *mut SYNC_RESOLVE_ACTION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChangeConflict_Impl::GetResolveActionForChange(this, core::mem::transmute_copy(&presolveaction)).into()
        }
        unsafe extern "system" fn SetResolveActionForChange<Identity: IChangeConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resolveaction: SYNC_RESOLVE_ACTION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChangeConflict_Impl::SetResolveActionForChange(this, core::mem::transmute_copy(&resolveaction)).into()
        }
        unsafe extern "system" fn GetResolveActionForChangeUnit<Identity: IChangeConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchangeunit: *mut core::ffi::c_void, presolveaction: *mut SYNC_RESOLVE_ACTION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChangeConflict_Impl::GetResolveActionForChangeUnit(this, windows_core::from_raw_borrowed(&pchangeunit), core::mem::transmute_copy(&presolveaction)).into()
        }
        unsafe extern "system" fn SetResolveActionForChangeUnit<Identity: IChangeConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchangeunit: *mut core::ffi::c_void, resolveaction: SYNC_RESOLVE_ACTION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChangeConflict_Impl::SetResolveActionForChangeUnit(this, windows_core::from_raw_borrowed(&pchangeunit), core::mem::transmute_copy(&resolveaction)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDestinationProviderConflictingChange: GetDestinationProviderConflictingChange::<Identity, OFFSET>,
            GetSourceProviderConflictingChange: GetSourceProviderConflictingChange::<Identity, OFFSET>,
            GetDestinationProviderConflictingData: GetDestinationProviderConflictingData::<Identity, OFFSET>,
            GetSourceProviderConflictingData: GetSourceProviderConflictingData::<Identity, OFFSET>,
            GetResolveActionForChange: GetResolveActionForChange::<Identity, OFFSET>,
            SetResolveActionForChange: SetResolveActionForChange::<Identity, OFFSET>,
            GetResolveActionForChangeUnit: GetResolveActionForChangeUnit::<Identity, OFFSET>,
            SetResolveActionForChangeUnit: SetResolveActionForChangeUnit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IChangeConflict as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IChangeUnitException, IChangeUnitException_Vtbl, 0x0cd7ee7c_fec0_4021_99ee_f0e5348f2a5f);
impl core::ops::Deref for IChangeUnitException {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IChangeUnitException, windows_core::IUnknown);
impl IChangeUnitException {
    pub unsafe fn GetItemId(&self, pbitemid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetItemId)(windows_core::Interface::as_raw(self), pbitemid, pcbidsize).ok()
    }
    pub unsafe fn GetChangeUnitId(&self, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetChangeUnitId)(windows_core::Interface::as_raw(self), pbchangeunitid, pcbidsize).ok()
    }
    pub unsafe fn GetClockVector(&self, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetClockVector)(windows_core::Interface::as_raw(self), riid, ppunk).ok()
    }
}
#[repr(C)]
pub struct IChangeUnitException_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetItemId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetChangeUnitId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetClockVector: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IChangeUnitException_Impl: Sized + windows_core::IUnknownImpl {
    fn GetItemId(&self, pbitemid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()>;
    fn GetChangeUnitId(&self, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()>;
    fn GetClockVector(&self, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IChangeUnitException {}
impl IChangeUnitException_Vtbl {
    pub const fn new<Identity: IChangeUnitException_Impl, const OFFSET: isize>() -> IChangeUnitException_Vtbl {
        unsafe extern "system" fn GetItemId<Identity: IChangeUnitException_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitemid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChangeUnitException_Impl::GetItemId(this, core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetChangeUnitId<Identity: IChangeUnitException_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChangeUnitException_Impl::GetChangeUnitId(this, core::mem::transmute_copy(&pbchangeunitid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetClockVector<Identity: IChangeUnitException_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChangeUnitException_Impl::GetClockVector(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetItemId: GetItemId::<Identity, OFFSET>,
            GetChangeUnitId: GetChangeUnitId::<Identity, OFFSET>,
            GetClockVector: GetClockVector::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IChangeUnitException as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IChangeUnitListFilterInfo, IChangeUnitListFilterInfo_Vtbl, 0xf2837671_0bdf_43fa_b502_232375fb50c2);
impl core::ops::Deref for IChangeUnitListFilterInfo {
    type Target = ISyncFilterInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IChangeUnitListFilterInfo, windows_core::IUnknown, ISyncFilterInfo);
impl IChangeUnitListFilterInfo {
    pub unsafe fn Initialize(&self, ppbchangeunitids: *const *const u8, dwchangeunitcount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), ppbchangeunitids, dwchangeunitcount).ok()
    }
    pub unsafe fn GetChangeUnitIdCount(&self, pdwchangeunitidcount: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetChangeUnitIdCount)(windows_core::Interface::as_raw(self), pdwchangeunitidcount).ok()
    }
    pub unsafe fn GetChangeUnitId(&self, dwchangeunitidindex: u32, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetChangeUnitId)(windows_core::Interface::as_raw(self), dwchangeunitidindex, pbchangeunitid, pcbidsize).ok()
    }
}
#[repr(C)]
pub struct IChangeUnitListFilterInfo_Vtbl {
    pub base__: ISyncFilterInfo_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *const *const u8, u32) -> windows_core::HRESULT,
    pub GetChangeUnitIdCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetChangeUnitId: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
}
pub trait IChangeUnitListFilterInfo_Impl: Sized + ISyncFilterInfo_Impl {
    fn Initialize(&self, ppbchangeunitids: *const *const u8, dwchangeunitcount: u32) -> windows_core::Result<()>;
    fn GetChangeUnitIdCount(&self, pdwchangeunitidcount: *mut u32) -> windows_core::Result<()>;
    fn GetChangeUnitId(&self, dwchangeunitidindex: u32, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IChangeUnitListFilterInfo {}
impl IChangeUnitListFilterInfo_Vtbl {
    pub const fn new<Identity: IChangeUnitListFilterInfo_Impl, const OFFSET: isize>() -> IChangeUnitListFilterInfo_Vtbl {
        unsafe extern "system" fn Initialize<Identity: IChangeUnitListFilterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbchangeunitids: *const *const u8, dwchangeunitcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChangeUnitListFilterInfo_Impl::Initialize(this, core::mem::transmute_copy(&ppbchangeunitids), core::mem::transmute_copy(&dwchangeunitcount)).into()
        }
        unsafe extern "system" fn GetChangeUnitIdCount<Identity: IChangeUnitListFilterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwchangeunitidcount: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChangeUnitListFilterInfo_Impl::GetChangeUnitIdCount(this, core::mem::transmute_copy(&pdwchangeunitidcount)).into()
        }
        unsafe extern "system" fn GetChangeUnitId<Identity: IChangeUnitListFilterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwchangeunitidindex: u32, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChangeUnitListFilterInfo_Impl::GetChangeUnitId(this, core::mem::transmute_copy(&dwchangeunitidindex), core::mem::transmute_copy(&pbchangeunitid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        Self {
            base__: ISyncFilterInfo_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            GetChangeUnitIdCount: GetChangeUnitIdCount::<Identity, OFFSET>,
            GetChangeUnitId: GetChangeUnitId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IChangeUnitListFilterInfo as windows_core::Interface>::IID || iid == &<ISyncFilterInfo as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IClockVector, IClockVector_Vtbl, 0x14b2274a_8698_4cc6_9333_f89bd1d47bc4);
impl core::ops::Deref for IClockVector {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IClockVector, windows_core::IUnknown);
impl IClockVector {
    pub unsafe fn GetClockVectorElements(&self, riid: *const windows_core::GUID, ppienumclockvector: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetClockVectorElements)(windows_core::Interface::as_raw(self), riid, ppienumclockvector).ok()
    }
    pub unsafe fn GetClockVectorElementCount(&self, pdwcount: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetClockVectorElementCount)(windows_core::Interface::as_raw(self), pdwcount).ok()
    }
}
#[repr(C)]
pub struct IClockVector_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetClockVectorElements: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetClockVectorElementCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IClockVector_Impl: Sized + windows_core::IUnknownImpl {
    fn GetClockVectorElements(&self, riid: *const windows_core::GUID, ppienumclockvector: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetClockVectorElementCount(&self, pdwcount: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IClockVector {}
impl IClockVector_Vtbl {
    pub const fn new<Identity: IClockVector_Impl, const OFFSET: isize>() -> IClockVector_Vtbl {
        unsafe extern "system" fn GetClockVectorElements<Identity: IClockVector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppienumclockvector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IClockVector_Impl::GetClockVectorElements(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppienumclockvector)).into()
        }
        unsafe extern "system" fn GetClockVectorElementCount<Identity: IClockVector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IClockVector_Impl::GetClockVectorElementCount(this, core::mem::transmute_copy(&pdwcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClockVectorElements: GetClockVectorElements::<Identity, OFFSET>,
            GetClockVectorElementCount: GetClockVectorElementCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClockVector as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IClockVectorElement, IClockVectorElement_Vtbl, 0xe71c4250_adf8_4a07_8fae_5669596909c1);
impl core::ops::Deref for IClockVectorElement {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IClockVectorElement, windows_core::IUnknown);
impl IClockVectorElement {
    pub unsafe fn GetReplicaKey(&self, pdwreplicakey: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetReplicaKey)(windows_core::Interface::as_raw(self), pdwreplicakey).ok()
    }
    pub unsafe fn GetTickCount(&self, pulltickcount: *mut u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTickCount)(windows_core::Interface::as_raw(self), pulltickcount).ok()
    }
}
#[repr(C)]
pub struct IClockVectorElement_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetReplicaKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetTickCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
pub trait IClockVectorElement_Impl: Sized + windows_core::IUnknownImpl {
    fn GetReplicaKey(&self, pdwreplicakey: *mut u32) -> windows_core::Result<()>;
    fn GetTickCount(&self, pulltickcount: *mut u64) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IClockVectorElement {}
impl IClockVectorElement_Vtbl {
    pub const fn new<Identity: IClockVectorElement_Impl, const OFFSET: isize>() -> IClockVectorElement_Vtbl {
        unsafe extern "system" fn GetReplicaKey<Identity: IClockVectorElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwreplicakey: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IClockVectorElement_Impl::GetReplicaKey(this, core::mem::transmute_copy(&pdwreplicakey)).into()
        }
        unsafe extern "system" fn GetTickCount<Identity: IClockVectorElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulltickcount: *mut u64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IClockVectorElement_Impl::GetTickCount(this, core::mem::transmute_copy(&pulltickcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetReplicaKey: GetReplicaKey::<Identity, OFFSET>,
            GetTickCount: GetTickCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClockVectorElement as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ICombinedFilterInfo, ICombinedFilterInfo_Vtbl, 0x11f9de71_2818_4779_b2ac_42d450565f45);
impl core::ops::Deref for ICombinedFilterInfo {
    type Target = ISyncFilterInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICombinedFilterInfo, windows_core::IUnknown, ISyncFilterInfo);
impl ICombinedFilterInfo {
    pub unsafe fn GetFilterCount(&self, pdwfiltercount: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFilterCount)(windows_core::Interface::as_raw(self), pdwfiltercount).ok()
    }
    pub unsafe fn GetFilterInfo(&self, dwfilterindex: u32) -> windows_core::Result<ISyncFilterInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFilterInfo)(windows_core::Interface::as_raw(self), dwfilterindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFilterCombinationType(&self, pfiltercombinationtype: *mut FILTER_COMBINATION_TYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFilterCombinationType)(windows_core::Interface::as_raw(self), pfiltercombinationtype).ok()
    }
}
#[repr(C)]
pub struct ICombinedFilterInfo_Vtbl {
    pub base__: ISyncFilterInfo_Vtbl,
    pub GetFilterCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetFilterInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFilterCombinationType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FILTER_COMBINATION_TYPE) -> windows_core::HRESULT,
}
pub trait ICombinedFilterInfo_Impl: Sized + ISyncFilterInfo_Impl {
    fn GetFilterCount(&self, pdwfiltercount: *mut u32) -> windows_core::Result<()>;
    fn GetFilterInfo(&self, dwfilterindex: u32) -> windows_core::Result<ISyncFilterInfo>;
    fn GetFilterCombinationType(&self, pfiltercombinationtype: *mut FILTER_COMBINATION_TYPE) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICombinedFilterInfo {}
impl ICombinedFilterInfo_Vtbl {
    pub const fn new<Identity: ICombinedFilterInfo_Impl, const OFFSET: isize>() -> ICombinedFilterInfo_Vtbl {
        unsafe extern "system" fn GetFilterCount<Identity: ICombinedFilterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwfiltercount: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICombinedFilterInfo_Impl::GetFilterCount(this, core::mem::transmute_copy(&pdwfiltercount)).into()
        }
        unsafe extern "system" fn GetFilterInfo<Identity: ICombinedFilterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfilterindex: u32, ppifilterinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICombinedFilterInfo_Impl::GetFilterInfo(this, core::mem::transmute_copy(&dwfilterindex)) {
                Ok(ok__) => {
                    ppifilterinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilterCombinationType<Identity: ICombinedFilterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfiltercombinationtype: *mut FILTER_COMBINATION_TYPE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICombinedFilterInfo_Impl::GetFilterCombinationType(this, core::mem::transmute_copy(&pfiltercombinationtype)).into()
        }
        Self {
            base__: ISyncFilterInfo_Vtbl::new::<Identity, OFFSET>(),
            GetFilterCount: GetFilterCount::<Identity, OFFSET>,
            GetFilterInfo: GetFilterInfo::<Identity, OFFSET>,
            GetFilterCombinationType: GetFilterCombinationType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICombinedFilterInfo as windows_core::Interface>::IID || iid == &<ISyncFilterInfo as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IConstraintConflict, IConstraintConflict_Vtbl, 0x00d2302e_1cf8_4835_b85f_b7ca4f799e0a);
impl core::ops::Deref for IConstraintConflict {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IConstraintConflict, windows_core::IUnknown);
impl IConstraintConflict {
    pub unsafe fn GetDestinationProviderConflictingChange(&self) -> windows_core::Result<ISyncChange> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDestinationProviderConflictingChange)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSourceProviderConflictingChange(&self) -> windows_core::Result<ISyncChange> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSourceProviderConflictingChange)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDestinationProviderOriginalChange(&self) -> windows_core::Result<ISyncChange> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDestinationProviderOriginalChange)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDestinationProviderConflictingData(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDestinationProviderConflictingData)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSourceProviderConflictingData(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSourceProviderConflictingData)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDestinationProviderOriginalData(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDestinationProviderOriginalData)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetConstraintResolveActionForChange(&self, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetConstraintResolveActionForChange)(windows_core::Interface::as_raw(self), pconstraintresolveaction).ok()
    }
    pub unsafe fn SetConstraintResolveActionForChange(&self, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetConstraintResolveActionForChange)(windows_core::Interface::as_raw(self), constraintresolveaction).ok()
    }
    pub unsafe fn GetConstraintResolveActionForChangeUnit<P0>(&self, pchangeunit: P0, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISyncChangeUnit>,
    {
        (windows_core::Interface::vtable(self).GetConstraintResolveActionForChangeUnit)(windows_core::Interface::as_raw(self), pchangeunit.param().abi(), pconstraintresolveaction).ok()
    }
    pub unsafe fn SetConstraintResolveActionForChangeUnit<P0>(&self, pchangeunit: P0, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISyncChangeUnit>,
    {
        (windows_core::Interface::vtable(self).SetConstraintResolveActionForChangeUnit)(windows_core::Interface::as_raw(self), pchangeunit.param().abi(), constraintresolveaction).ok()
    }
    pub unsafe fn GetConstraintConflictReason(&self, pconstraintconflictreason: *mut CONSTRAINT_CONFLICT_REASON) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetConstraintConflictReason)(windows_core::Interface::as_raw(self), pconstraintconflictreason).ok()
    }
    pub unsafe fn IsTemporary(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsTemporary)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IConstraintConflict_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDestinationProviderConflictingChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSourceProviderConflictingChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDestinationProviderOriginalChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDestinationProviderConflictingData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSourceProviderConflictingData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDestinationProviderOriginalData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetConstraintResolveActionForChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> windows_core::HRESULT,
    pub SetConstraintResolveActionForChange: unsafe extern "system" fn(*mut core::ffi::c_void, SYNC_CONSTRAINT_RESOLVE_ACTION) -> windows_core::HRESULT,
    pub GetConstraintResolveActionForChangeUnit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> windows_core::HRESULT,
    pub SetConstraintResolveActionForChangeUnit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, SYNC_CONSTRAINT_RESOLVE_ACTION) -> windows_core::HRESULT,
    pub GetConstraintConflictReason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CONSTRAINT_CONFLICT_REASON) -> windows_core::HRESULT,
    pub IsTemporary: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IConstraintConflict_Impl: Sized + windows_core::IUnknownImpl {
    fn GetDestinationProviderConflictingChange(&self) -> windows_core::Result<ISyncChange>;
    fn GetSourceProviderConflictingChange(&self) -> windows_core::Result<ISyncChange>;
    fn GetDestinationProviderOriginalChange(&self) -> windows_core::Result<ISyncChange>;
    fn GetDestinationProviderConflictingData(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetSourceProviderConflictingData(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetDestinationProviderOriginalData(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetConstraintResolveActionForChange(&self, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> windows_core::Result<()>;
    fn SetConstraintResolveActionForChange(&self, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> windows_core::Result<()>;
    fn GetConstraintResolveActionForChangeUnit(&self, pchangeunit: Option<&ISyncChangeUnit>, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> windows_core::Result<()>;
    fn SetConstraintResolveActionForChangeUnit(&self, pchangeunit: Option<&ISyncChangeUnit>, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> windows_core::Result<()>;
    fn GetConstraintConflictReason(&self, pconstraintconflictreason: *mut CONSTRAINT_CONFLICT_REASON) -> windows_core::Result<()>;
    fn IsTemporary(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IConstraintConflict {}
impl IConstraintConflict_Vtbl {
    pub const fn new<Identity: IConstraintConflict_Impl, const OFFSET: isize>() -> IConstraintConflict_Vtbl {
        unsafe extern "system" fn GetDestinationProviderConflictingChange<Identity: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppconflictingchange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConstraintConflict_Impl::GetDestinationProviderConflictingChange(this) {
                Ok(ok__) => {
                    ppconflictingchange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceProviderConflictingChange<Identity: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppconflictingchange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConstraintConflict_Impl::GetSourceProviderConflictingChange(this) {
                Ok(ok__) => {
                    ppconflictingchange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestinationProviderOriginalChange<Identity: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pporiginalchange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConstraintConflict_Impl::GetDestinationProviderOriginalChange(this) {
                Ok(ok__) => {
                    pporiginalchange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestinationProviderConflictingData<Identity: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppconflictingdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConstraintConflict_Impl::GetDestinationProviderConflictingData(this) {
                Ok(ok__) => {
                    ppconflictingdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceProviderConflictingData<Identity: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppconflictingdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConstraintConflict_Impl::GetSourceProviderConflictingData(this) {
                Ok(ok__) => {
                    ppconflictingdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestinationProviderOriginalData<Identity: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pporiginaldata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConstraintConflict_Impl::GetDestinationProviderOriginalData(this) {
                Ok(ok__) => {
                    pporiginaldata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstraintResolveActionForChange<Identity: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConstraintConflict_Impl::GetConstraintResolveActionForChange(this, core::mem::transmute_copy(&pconstraintresolveaction)).into()
        }
        unsafe extern "system" fn SetConstraintResolveActionForChange<Identity: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConstraintConflict_Impl::SetConstraintResolveActionForChange(this, core::mem::transmute_copy(&constraintresolveaction)).into()
        }
        unsafe extern "system" fn GetConstraintResolveActionForChangeUnit<Identity: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchangeunit: *mut core::ffi::c_void, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConstraintConflict_Impl::GetConstraintResolveActionForChangeUnit(this, windows_core::from_raw_borrowed(&pchangeunit), core::mem::transmute_copy(&pconstraintresolveaction)).into()
        }
        unsafe extern "system" fn SetConstraintResolveActionForChangeUnit<Identity: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchangeunit: *mut core::ffi::c_void, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConstraintConflict_Impl::SetConstraintResolveActionForChangeUnit(this, windows_core::from_raw_borrowed(&pchangeunit), core::mem::transmute_copy(&constraintresolveaction)).into()
        }
        unsafe extern "system" fn GetConstraintConflictReason<Identity: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconstraintconflictreason: *mut CONSTRAINT_CONFLICT_REASON) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConstraintConflict_Impl::GetConstraintConflictReason(this, core::mem::transmute_copy(&pconstraintconflictreason)).into()
        }
        unsafe extern "system" fn IsTemporary<Identity: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConstraintConflict_Impl::IsTemporary(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDestinationProviderConflictingChange: GetDestinationProviderConflictingChange::<Identity, OFFSET>,
            GetSourceProviderConflictingChange: GetSourceProviderConflictingChange::<Identity, OFFSET>,
            GetDestinationProviderOriginalChange: GetDestinationProviderOriginalChange::<Identity, OFFSET>,
            GetDestinationProviderConflictingData: GetDestinationProviderConflictingData::<Identity, OFFSET>,
            GetSourceProviderConflictingData: GetSourceProviderConflictingData::<Identity, OFFSET>,
            GetDestinationProviderOriginalData: GetDestinationProviderOriginalData::<Identity, OFFSET>,
            GetConstraintResolveActionForChange: GetConstraintResolveActionForChange::<Identity, OFFSET>,
            SetConstraintResolveActionForChange: SetConstraintResolveActionForChange::<Identity, OFFSET>,
            GetConstraintResolveActionForChangeUnit: GetConstraintResolveActionForChangeUnit::<Identity, OFFSET>,
            SetConstraintResolveActionForChangeUnit: SetConstraintResolveActionForChangeUnit::<Identity, OFFSET>,
            GetConstraintConflictReason: GetConstraintConflictReason::<Identity, OFFSET>,
            IsTemporary: IsTemporary::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IConstraintConflict as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IConstructReplicaKeyMap, IConstructReplicaKeyMap_Vtbl, 0xded10970_ec85_4115_b52c_4405845642a5);
impl core::ops::Deref for IConstructReplicaKeyMap {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IConstructReplicaKeyMap, windows_core::IUnknown);
impl IConstructReplicaKeyMap {
    pub unsafe fn FindOrAddReplica(&self, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FindOrAddReplica)(windows_core::Interface::as_raw(self), pbreplicaid, pdwreplicakey).ok()
    }
}
#[repr(C)]
pub struct IConstructReplicaKeyMap_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FindOrAddReplica: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *mut u32) -> windows_core::HRESULT,
}
pub trait IConstructReplicaKeyMap_Impl: Sized + windows_core::IUnknownImpl {
    fn FindOrAddReplica(&self, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IConstructReplicaKeyMap {}
impl IConstructReplicaKeyMap_Vtbl {
    pub const fn new<Identity: IConstructReplicaKeyMap_Impl, const OFFSET: isize>() -> IConstructReplicaKeyMap_Vtbl {
        unsafe extern "system" fn FindOrAddReplica<Identity: IConstructReplicaKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConstructReplicaKeyMap_Impl::FindOrAddReplica(this, core::mem::transmute_copy(&pbreplicaid), core::mem::transmute_copy(&pdwreplicakey)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), FindOrAddReplica: FindOrAddReplica::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IConstructReplicaKeyMap as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ICoreFragment, ICoreFragment_Vtbl, 0x613b2ab5_b304_47d9_9c31_ce6c54401a15);
impl core::ops::Deref for ICoreFragment {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICoreFragment, windows_core::IUnknown);
impl ICoreFragment {
    pub unsafe fn NextColumn(&self, pchangeunitid: *mut u8, pchangeunitidsize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NextColumn)(windows_core::Interface::as_raw(self), pchangeunitid, pchangeunitidsize).ok()
    }
    pub unsafe fn NextRange(&self, pitemid: *mut u8, pitemidsize: *mut u32, piclockvector: *mut Option<IClockVector>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NextRange)(windows_core::Interface::as_raw(self), pitemid, pitemidsize, core::mem::transmute(piclockvector)).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetColumnCount(&self, pcolumncount: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetColumnCount)(windows_core::Interface::as_raw(self), pcolumncount).ok()
    }
    pub unsafe fn GetRangeCount(&self, prangecount: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRangeCount)(windows_core::Interface::as_raw(self), prangecount).ok()
    }
}
#[repr(C)]
pub struct ICoreFragment_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NextColumn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub NextRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetColumnCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetRangeCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait ICoreFragment_Impl: Sized + windows_core::IUnknownImpl {
    fn NextColumn(&self, pchangeunitid: *mut u8, pchangeunitidsize: *mut u32) -> windows_core::Result<()>;
    fn NextRange(&self, pitemid: *mut u8, pitemidsize: *mut u32, piclockvector: *mut Option<IClockVector>) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn GetColumnCount(&self, pcolumncount: *mut u32) -> windows_core::Result<()>;
    fn GetRangeCount(&self, prangecount: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICoreFragment {}
impl ICoreFragment_Vtbl {
    pub const fn new<Identity: ICoreFragment_Impl, const OFFSET: isize>() -> ICoreFragment_Vtbl {
        unsafe extern "system" fn NextColumn<Identity: ICoreFragment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchangeunitid: *mut u8, pchangeunitidsize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreFragment_Impl::NextColumn(this, core::mem::transmute_copy(&pchangeunitid), core::mem::transmute_copy(&pchangeunitidsize)).into()
        }
        unsafe extern "system" fn NextRange<Identity: ICoreFragment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitemid: *mut u8, pitemidsize: *mut u32, piclockvector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreFragment_Impl::NextRange(this, core::mem::transmute_copy(&pitemid), core::mem::transmute_copy(&pitemidsize), core::mem::transmute_copy(&piclockvector)).into()
        }
        unsafe extern "system" fn Reset<Identity: ICoreFragment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreFragment_Impl::Reset(this).into()
        }
        unsafe extern "system" fn GetColumnCount<Identity: ICoreFragment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolumncount: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreFragment_Impl::GetColumnCount(this, core::mem::transmute_copy(&pcolumncount)).into()
        }
        unsafe extern "system" fn GetRangeCount<Identity: ICoreFragment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prangecount: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreFragment_Impl::GetRangeCount(this, core::mem::transmute_copy(&prangecount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NextColumn: NextColumn::<Identity, OFFSET>,
            NextRange: NextRange::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            GetColumnCount: GetColumnCount::<Identity, OFFSET>,
            GetRangeCount: GetRangeCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreFragment as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ICoreFragmentInspector, ICoreFragmentInspector_Vtbl, 0xf7fcc5fd_ae26_4679_ba16_96aac583c134);
impl core::ops::Deref for ICoreFragmentInspector {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICoreFragmentInspector, windows_core::IUnknown);
impl ICoreFragmentInspector {
    pub unsafe fn NextCoreFragments(&self, requestedcount: u32, ppicorefragments: *mut Option<ICoreFragment>, pfetchedcount: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NextCoreFragments)(windows_core::Interface::as_raw(self), requestedcount, core::mem::transmute(ppicorefragments), pfetchedcount).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ICoreFragmentInspector_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NextCoreFragments: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICoreFragmentInspector_Impl: Sized + windows_core::IUnknownImpl {
    fn NextCoreFragments(&self, requestedcount: u32, ppicorefragments: *mut Option<ICoreFragment>, pfetchedcount: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICoreFragmentInspector {}
impl ICoreFragmentInspector_Vtbl {
    pub const fn new<Identity: ICoreFragmentInspector_Impl, const OFFSET: isize>() -> ICoreFragmentInspector_Vtbl {
        unsafe extern "system" fn NextCoreFragments<Identity: ICoreFragmentInspector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestedcount: u32, ppicorefragments: *mut *mut core::ffi::c_void, pfetchedcount: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreFragmentInspector_Impl::NextCoreFragments(this, core::mem::transmute_copy(&requestedcount), core::mem::transmute_copy(&ppicorefragments), core::mem::transmute_copy(&pfetchedcount)).into()
        }
        unsafe extern "system" fn Reset<Identity: ICoreFragmentInspector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreFragmentInspector_Impl::Reset(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NextCoreFragments: NextCoreFragments::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreFragmentInspector as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ICustomFilterInfo, ICustomFilterInfo_Vtbl, 0x1d335dff_6f88_4e4d_91a8_a3f351cfd473);
impl core::ops::Deref for ICustomFilterInfo {
    type Target = ISyncFilterInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICustomFilterInfo, windows_core::IUnknown, ISyncFilterInfo);
impl ICustomFilterInfo {
    pub unsafe fn GetSyncFilter(&self) -> windows_core::Result<ISyncFilter> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSyncFilter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ICustomFilterInfo_Vtbl {
    pub base__: ISyncFilterInfo_Vtbl,
    pub GetSyncFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICustomFilterInfo_Impl: Sized + ISyncFilterInfo_Impl {
    fn GetSyncFilter(&self) -> windows_core::Result<ISyncFilter>;
}
impl windows_core::RuntimeName for ICustomFilterInfo {}
impl ICustomFilterInfo_Vtbl {
    pub const fn new<Identity: ICustomFilterInfo_Impl, const OFFSET: isize>() -> ICustomFilterInfo_Vtbl {
        unsafe extern "system" fn GetSyncFilter<Identity: ICustomFilterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisyncfilter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICustomFilterInfo_Impl::GetSyncFilter(this) {
                Ok(ok__) => {
                    pisyncfilter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ISyncFilterInfo_Vtbl::new::<Identity, OFFSET>(), GetSyncFilter: GetSyncFilter::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICustomFilterInfo as windows_core::Interface>::IID || iid == &<ISyncFilterInfo as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IDataRetrieverCallback, IDataRetrieverCallback_Vtbl, 0x71b4863b_f969_4676_bbc3_3d9fdc3fb2c7);
impl core::ops::Deref for IDataRetrieverCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDataRetrieverCallback, windows_core::IUnknown);
impl IDataRetrieverCallback {
    pub unsafe fn LoadChangeDataComplete<P0>(&self, punkdata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).LoadChangeDataComplete)(windows_core::Interface::as_raw(self), punkdata.param().abi()).ok()
    }
    pub unsafe fn LoadChangeDataError(&self, hrerror: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LoadChangeDataError)(windows_core::Interface::as_raw(self), hrerror).ok()
    }
}
#[repr(C)]
pub struct IDataRetrieverCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub LoadChangeDataComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LoadChangeDataError: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait IDataRetrieverCallback_Impl: Sized + windows_core::IUnknownImpl {
    fn LoadChangeDataComplete(&self, punkdata: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn LoadChangeDataError(&self, hrerror: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDataRetrieverCallback {}
impl IDataRetrieverCallback_Vtbl {
    pub const fn new<Identity: IDataRetrieverCallback_Impl, const OFFSET: isize>() -> IDataRetrieverCallback_Vtbl {
        unsafe extern "system" fn LoadChangeDataComplete<Identity: IDataRetrieverCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataRetrieverCallback_Impl::LoadChangeDataComplete(this, windows_core::from_raw_borrowed(&punkdata)).into()
        }
        unsafe extern "system" fn LoadChangeDataError<Identity: IDataRetrieverCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrerror: windows_core::HRESULT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataRetrieverCallback_Impl::LoadChangeDataError(this, core::mem::transmute_copy(&hrerror)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LoadChangeDataComplete: LoadChangeDataComplete::<Identity, OFFSET>,
            LoadChangeDataError: LoadChangeDataError::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataRetrieverCallback as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IEnumChangeUnitExceptions, IEnumChangeUnitExceptions_Vtbl, 0x3074e802_9319_4420_be21_1022e2e21da8);
impl core::ops::Deref for IEnumChangeUnitExceptions {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumChangeUnitExceptions, windows_core::IUnknown);
impl IEnumChangeUnitExceptions {
    pub unsafe fn Next(&self, cexceptions: u32, ppchangeunitexception: *mut Option<IChangeUnitException>, pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), cexceptions, core::mem::transmute(ppchangeunitexception), pcfetched).ok()
    }
    pub unsafe fn Skip(&self, cexceptions: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), cexceptions).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumChangeUnitExceptions> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumChangeUnitExceptions_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumChangeUnitExceptions_Impl: Sized + windows_core::IUnknownImpl {
    fn Next(&self, cexceptions: u32, ppchangeunitexception: *mut Option<IChangeUnitException>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, cexceptions: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumChangeUnitExceptions>;
}
impl windows_core::RuntimeName for IEnumChangeUnitExceptions {}
impl IEnumChangeUnitExceptions_Vtbl {
    pub const fn new<Identity: IEnumChangeUnitExceptions_Impl, const OFFSET: isize>() -> IEnumChangeUnitExceptions_Vtbl {
        unsafe extern "system" fn Next<Identity: IEnumChangeUnitExceptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cexceptions: u32, ppchangeunitexception: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumChangeUnitExceptions_Impl::Next(this, core::mem::transmute_copy(&cexceptions), core::mem::transmute_copy(&ppchangeunitexception), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: IEnumChangeUnitExceptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cexceptions: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumChangeUnitExceptions_Impl::Skip(this, core::mem::transmute_copy(&cexceptions)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnumChangeUnitExceptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumChangeUnitExceptions_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: IEnumChangeUnitExceptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumChangeUnitExceptions_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumChangeUnitExceptions as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IEnumClockVector, IEnumClockVector_Vtbl, 0x525844db_2837_4799_9e80_81a66e02220c);
impl core::ops::Deref for IEnumClockVector {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumClockVector, windows_core::IUnknown);
impl IEnumClockVector {
    pub unsafe fn Next(&self, cclockvectorelements: u32, ppiclockvectorelements: *mut Option<IClockVectorElement>, pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), cclockvectorelements, core::mem::transmute(ppiclockvectorelements), pcfetched).ok()
    }
    pub unsafe fn Skip(&self, csyncversions: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), csyncversions).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumClockVector> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumClockVector_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumClockVector_Impl: Sized + windows_core::IUnknownImpl {
    fn Next(&self, cclockvectorelements: u32, ppiclockvectorelements: *mut Option<IClockVectorElement>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, csyncversions: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumClockVector>;
}
impl windows_core::RuntimeName for IEnumClockVector {}
impl IEnumClockVector_Vtbl {
    pub const fn new<Identity: IEnumClockVector_Impl, const OFFSET: isize>() -> IEnumClockVector_Vtbl {
        unsafe extern "system" fn Next<Identity: IEnumClockVector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cclockvectorelements: u32, ppiclockvectorelements: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumClockVector_Impl::Next(this, core::mem::transmute_copy(&cclockvectorelements), core::mem::transmute_copy(&ppiclockvectorelements), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: IEnumClockVector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, csyncversions: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumClockVector_Impl::Skip(this, core::mem::transmute_copy(&csyncversions)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnumClockVector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumClockVector_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: IEnumClockVector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumClockVector_Impl::Clone(this) {
                Ok(ok__) => {
                    ppienum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumClockVector as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IEnumFeedClockVector, IEnumFeedClockVector_Vtbl, 0x550f763d_146a_48f6_abeb_6c88c7f70514);
impl core::ops::Deref for IEnumFeedClockVector {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumFeedClockVector, windows_core::IUnknown);
impl IEnumFeedClockVector {
    pub unsafe fn Next(&self, cclockvectorelements: u32, ppiclockvectorelements: *mut Option<IFeedClockVectorElement>, pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), cclockvectorelements, core::mem::transmute(ppiclockvectorelements), pcfetched).ok()
    }
    pub unsafe fn Skip(&self, csyncversions: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), csyncversions).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumFeedClockVector> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumFeedClockVector_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumFeedClockVector_Impl: Sized + windows_core::IUnknownImpl {
    fn Next(&self, cclockvectorelements: u32, ppiclockvectorelements: *mut Option<IFeedClockVectorElement>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, csyncversions: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumFeedClockVector>;
}
impl windows_core::RuntimeName for IEnumFeedClockVector {}
impl IEnumFeedClockVector_Vtbl {
    pub const fn new<Identity: IEnumFeedClockVector_Impl, const OFFSET: isize>() -> IEnumFeedClockVector_Vtbl {
        unsafe extern "system" fn Next<Identity: IEnumFeedClockVector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cclockvectorelements: u32, ppiclockvectorelements: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumFeedClockVector_Impl::Next(this, core::mem::transmute_copy(&cclockvectorelements), core::mem::transmute_copy(&ppiclockvectorelements), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: IEnumFeedClockVector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, csyncversions: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumFeedClockVector_Impl::Skip(this, core::mem::transmute_copy(&csyncversions)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnumFeedClockVector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumFeedClockVector_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: IEnumFeedClockVector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumFeedClockVector_Impl::Clone(this) {
                Ok(ok__) => {
                    ppienum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumFeedClockVector as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IEnumItemIds, IEnumItemIds_Vtbl, 0x43aa3f61_4b2e_4b60_83df_b110d3e148f1);
impl core::ops::Deref for IEnumItemIds {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumItemIds, windows_core::IUnknown);
impl IEnumItemIds {
    pub unsafe fn Next(&self, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), pbitemid, pcbitemidsize).ok()
    }
}
#[repr(C)]
pub struct IEnumItemIds_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
}
pub trait IEnumItemIds_Impl: Sized + windows_core::IUnknownImpl {
    fn Next(&self, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEnumItemIds {}
impl IEnumItemIds_Vtbl {
    pub const fn new<Identity: IEnumItemIds_Impl, const OFFSET: isize>() -> IEnumItemIds_Vtbl {
        unsafe extern "system" fn Next<Identity: IEnumItemIds_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumItemIds_Impl::Next(this, core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pcbitemidsize)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumItemIds as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IEnumRangeExceptions, IEnumRangeExceptions_Vtbl, 0x0944439f_ddb1_4176_b703_046ff22a2386);
impl core::ops::Deref for IEnumRangeExceptions {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumRangeExceptions, windows_core::IUnknown);
impl IEnumRangeExceptions {
    pub unsafe fn Next(&self, cexceptions: u32, pprangeexception: *mut Option<IRangeException>, pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), cexceptions, core::mem::transmute(pprangeexception), pcfetched).ok()
    }
    pub unsafe fn Skip(&self, cexceptions: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), cexceptions).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumRangeExceptions> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumRangeExceptions_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumRangeExceptions_Impl: Sized + windows_core::IUnknownImpl {
    fn Next(&self, cexceptions: u32, pprangeexception: *mut Option<IRangeException>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, cexceptions: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumRangeExceptions>;
}
impl windows_core::RuntimeName for IEnumRangeExceptions {}
impl IEnumRangeExceptions_Vtbl {
    pub const fn new<Identity: IEnumRangeExceptions_Impl, const OFFSET: isize>() -> IEnumRangeExceptions_Vtbl {
        unsafe extern "system" fn Next<Identity: IEnumRangeExceptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cexceptions: u32, pprangeexception: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumRangeExceptions_Impl::Next(this, core::mem::transmute_copy(&cexceptions), core::mem::transmute_copy(&pprangeexception), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: IEnumRangeExceptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cexceptions: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumRangeExceptions_Impl::Skip(this, core::mem::transmute_copy(&cexceptions)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnumRangeExceptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumRangeExceptions_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: IEnumRangeExceptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumRangeExceptions_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumRangeExceptions as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IEnumSingleItemExceptions, IEnumSingleItemExceptions_Vtbl, 0xe563381c_1b4d_4c66_9796_c86faccdcd40);
impl core::ops::Deref for IEnumSingleItemExceptions {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumSingleItemExceptions, windows_core::IUnknown);
impl IEnumSingleItemExceptions {
    pub unsafe fn Next(&self, cexceptions: u32, ppsingleitemexception: *mut Option<ISingleItemException>, pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), cexceptions, core::mem::transmute(ppsingleitemexception), pcfetched).ok()
    }
    pub unsafe fn Skip(&self, cexceptions: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), cexceptions).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumSingleItemExceptions> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumSingleItemExceptions_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumSingleItemExceptions_Impl: Sized + windows_core::IUnknownImpl {
    fn Next(&self, cexceptions: u32, ppsingleitemexception: *mut Option<ISingleItemException>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, cexceptions: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumSingleItemExceptions>;
}
impl windows_core::RuntimeName for IEnumSingleItemExceptions {}
impl IEnumSingleItemExceptions_Vtbl {
    pub const fn new<Identity: IEnumSingleItemExceptions_Impl, const OFFSET: isize>() -> IEnumSingleItemExceptions_Vtbl {
        unsafe extern "system" fn Next<Identity: IEnumSingleItemExceptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cexceptions: u32, ppsingleitemexception: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSingleItemExceptions_Impl::Next(this, core::mem::transmute_copy(&cexceptions), core::mem::transmute_copy(&ppsingleitemexception), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: IEnumSingleItemExceptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cexceptions: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSingleItemExceptions_Impl::Skip(this, core::mem::transmute_copy(&cexceptions)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnumSingleItemExceptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSingleItemExceptions_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: IEnumSingleItemExceptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumSingleItemExceptions_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumSingleItemExceptions as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IEnumSyncChangeUnits, IEnumSyncChangeUnits_Vtbl, 0x346b35f1_8703_4c6d_ab1a_4dbca2cff97f);
impl core::ops::Deref for IEnumSyncChangeUnits {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumSyncChangeUnits, windows_core::IUnknown);
impl IEnumSyncChangeUnits {
    pub unsafe fn Next(&self, cchanges: u32, ppchangeunit: *mut Option<ISyncChangeUnit>, pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), cchanges, core::mem::transmute(ppchangeunit), pcfetched).ok()
    }
    pub unsafe fn Skip(&self, cchanges: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), cchanges).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumSyncChangeUnits> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumSyncChangeUnits_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumSyncChangeUnits_Impl: Sized + windows_core::IUnknownImpl {
    fn Next(&self, cchanges: u32, ppchangeunit: *mut Option<ISyncChangeUnit>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, cchanges: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumSyncChangeUnits>;
}
impl windows_core::RuntimeName for IEnumSyncChangeUnits {}
impl IEnumSyncChangeUnits_Vtbl {
    pub const fn new<Identity: IEnumSyncChangeUnits_Impl, const OFFSET: isize>() -> IEnumSyncChangeUnits_Vtbl {
        unsafe extern "system" fn Next<Identity: IEnumSyncChangeUnits_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchanges: u32, ppchangeunit: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSyncChangeUnits_Impl::Next(this, core::mem::transmute_copy(&cchanges), core::mem::transmute_copy(&ppchangeunit), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: IEnumSyncChangeUnits_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchanges: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSyncChangeUnits_Impl::Skip(this, core::mem::transmute_copy(&cchanges)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnumSyncChangeUnits_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSyncChangeUnits_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: IEnumSyncChangeUnits_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumSyncChangeUnits_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumSyncChangeUnits as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IEnumSyncChanges, IEnumSyncChanges_Vtbl, 0x5f86be4a_5e78_4e32_ac1c_c24fd223ef85);
impl core::ops::Deref for IEnumSyncChanges {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumSyncChanges, windows_core::IUnknown);
impl IEnumSyncChanges {
    pub unsafe fn Next(&self, cchanges: u32, ppchange: *mut Option<ISyncChange>, pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), cchanges, core::mem::transmute(ppchange), pcfetched).ok()
    }
    pub unsafe fn Skip(&self, cchanges: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), cchanges).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumSyncChanges> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumSyncChanges_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumSyncChanges_Impl: Sized + windows_core::IUnknownImpl {
    fn Next(&self, cchanges: u32, ppchange: *mut Option<ISyncChange>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, cchanges: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumSyncChanges>;
}
impl windows_core::RuntimeName for IEnumSyncChanges {}
impl IEnumSyncChanges_Vtbl {
    pub const fn new<Identity: IEnumSyncChanges_Impl, const OFFSET: isize>() -> IEnumSyncChanges_Vtbl {
        unsafe extern "system" fn Next<Identity: IEnumSyncChanges_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchanges: u32, ppchange: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSyncChanges_Impl::Next(this, core::mem::transmute_copy(&cchanges), core::mem::transmute_copy(&ppchange), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: IEnumSyncChanges_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchanges: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSyncChanges_Impl::Skip(this, core::mem::transmute_copy(&cchanges)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnumSyncChanges_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSyncChanges_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: IEnumSyncChanges_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumSyncChanges_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumSyncChanges as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IEnumSyncProviderConfigUIInfos, IEnumSyncProviderConfigUIInfos_Vtbl, 0xf6be2602_17c6_4658_a2d7_68ed3330f641);
impl core::ops::Deref for IEnumSyncProviderConfigUIInfos {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumSyncProviderConfigUIInfos, windows_core::IUnknown);
impl IEnumSyncProviderConfigUIInfos {
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Next(&self, ppsyncproviderconfiguiinfo: &mut [Option<ISyncProviderConfigUIInfo>], pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppsyncproviderconfiguiinfo.len().try_into().unwrap(), core::mem::transmute(ppsyncproviderconfiguiinfo.as_ptr()), pcfetched).ok()
    }
    pub unsafe fn Skip(&self, cfactories: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), cfactories).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumSyncProviderConfigUIInfos> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumSyncProviderConfigUIInfos_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IEnumSyncProviderConfigUIInfos_Impl: Sized + windows_core::IUnknownImpl {
    fn Next(&self, cfactories: u32, ppsyncproviderconfiguiinfo: *mut Option<ISyncProviderConfigUIInfo>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, cfactories: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumSyncProviderConfigUIInfos>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for IEnumSyncProviderConfigUIInfos {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IEnumSyncProviderConfigUIInfos_Vtbl {
    pub const fn new<Identity: IEnumSyncProviderConfigUIInfos_Impl, const OFFSET: isize>() -> IEnumSyncProviderConfigUIInfos_Vtbl {
        unsafe extern "system" fn Next<Identity: IEnumSyncProviderConfigUIInfos_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cfactories: u32, ppsyncproviderconfiguiinfo: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSyncProviderConfigUIInfos_Impl::Next(this, core::mem::transmute_copy(&cfactories), core::mem::transmute_copy(&ppsyncproviderconfiguiinfo), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: IEnumSyncProviderConfigUIInfos_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cfactories: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSyncProviderConfigUIInfos_Impl::Skip(this, core::mem::transmute_copy(&cfactories)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnumSyncProviderConfigUIInfos_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSyncProviderConfigUIInfos_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: IEnumSyncProviderConfigUIInfos_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumSyncProviderConfigUIInfos_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumSyncProviderConfigUIInfos as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IEnumSyncProviderInfos, IEnumSyncProviderInfos_Vtbl, 0xa04ba850_5eb1_460d_a973_393fcb608a11);
impl core::ops::Deref for IEnumSyncProviderInfos {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumSyncProviderInfos, windows_core::IUnknown);
impl IEnumSyncProviderInfos {
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Next(&self, ppsyncproviderinfo: &mut [Option<ISyncProviderInfo>], pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppsyncproviderinfo.len().try_into().unwrap(), core::mem::transmute(ppsyncproviderinfo.as_ptr()), pcfetched).ok()
    }
    pub unsafe fn Skip(&self, cinstances: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), cinstances).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumSyncProviderInfos> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumSyncProviderInfos_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IEnumSyncProviderInfos_Impl: Sized + windows_core::IUnknownImpl {
    fn Next(&self, cinstances: u32, ppsyncproviderinfo: *mut Option<ISyncProviderInfo>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, cinstances: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumSyncProviderInfos>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for IEnumSyncProviderInfos {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IEnumSyncProviderInfos_Vtbl {
    pub const fn new<Identity: IEnumSyncProviderInfos_Impl, const OFFSET: isize>() -> IEnumSyncProviderInfos_Vtbl {
        unsafe extern "system" fn Next<Identity: IEnumSyncProviderInfos_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cinstances: u32, ppsyncproviderinfo: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSyncProviderInfos_Impl::Next(this, core::mem::transmute_copy(&cinstances), core::mem::transmute_copy(&ppsyncproviderinfo), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: IEnumSyncProviderInfos_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cinstances: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSyncProviderInfos_Impl::Skip(this, core::mem::transmute_copy(&cinstances)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnumSyncProviderInfos_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSyncProviderInfos_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: IEnumSyncProviderInfos_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumSyncProviderInfos_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumSyncProviderInfos as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IFeedClockVector, IFeedClockVector_Vtbl, 0x8d1d98d1_9fb8_4ec9_a553_54dd924e0f67);
impl core::ops::Deref for IFeedClockVector {
    type Target = IClockVector;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFeedClockVector, windows_core::IUnknown, IClockVector);
impl IFeedClockVector {
    pub unsafe fn GetUpdateCount(&self, pdwupdatecount: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetUpdateCount)(windows_core::Interface::as_raw(self), pdwupdatecount).ok()
    }
    pub unsafe fn IsNoConflictsSpecified(&self, pfisnoconflictsspecified: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsNoConflictsSpecified)(windows_core::Interface::as_raw(self), pfisnoconflictsspecified).ok()
    }
}
#[repr(C)]
pub struct IFeedClockVector_Vtbl {
    pub base__: IClockVector_Vtbl,
    pub GetUpdateCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub IsNoConflictsSpecified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
pub trait IFeedClockVector_Impl: Sized + IClockVector_Impl {
    fn GetUpdateCount(&self, pdwupdatecount: *mut u32) -> windows_core::Result<()>;
    fn IsNoConflictsSpecified(&self, pfisnoconflictsspecified: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IFeedClockVector {}
impl IFeedClockVector_Vtbl {
    pub const fn new<Identity: IFeedClockVector_Impl, const OFFSET: isize>() -> IFeedClockVector_Vtbl {
        unsafe extern "system" fn GetUpdateCount<Identity: IFeedClockVector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwupdatecount: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedClockVector_Impl::GetUpdateCount(this, core::mem::transmute_copy(&pdwupdatecount)).into()
        }
        unsafe extern "system" fn IsNoConflictsSpecified<Identity: IFeedClockVector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisnoconflictsspecified: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedClockVector_Impl::IsNoConflictsSpecified(this, core::mem::transmute_copy(&pfisnoconflictsspecified)).into()
        }
        Self {
            base__: IClockVector_Vtbl::new::<Identity, OFFSET>(),
            GetUpdateCount: GetUpdateCount::<Identity, OFFSET>,
            IsNoConflictsSpecified: IsNoConflictsSpecified::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFeedClockVector as windows_core::Interface>::IID || iid == &<IClockVector as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IFeedClockVectorElement, IFeedClockVectorElement_Vtbl, 0xa40b46d2_e97b_4156_b6da_991f501b0f05);
impl core::ops::Deref for IFeedClockVectorElement {
    type Target = IClockVectorElement;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFeedClockVectorElement, windows_core::IUnknown, IClockVectorElement);
impl IFeedClockVectorElement {
    pub unsafe fn GetSyncTime(&self, psynctime: *mut SYNC_TIME) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSyncTime)(windows_core::Interface::as_raw(self), psynctime).ok()
    }
    pub unsafe fn GetFlags(&self, pbflags: *mut u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFlags)(windows_core::Interface::as_raw(self), pbflags).ok()
    }
}
#[repr(C)]
pub struct IFeedClockVectorElement_Vtbl {
    pub base__: IClockVectorElement_Vtbl,
    pub GetSyncTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SYNC_TIME) -> windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
}
pub trait IFeedClockVectorElement_Impl: Sized + IClockVectorElement_Impl {
    fn GetSyncTime(&self, psynctime: *mut SYNC_TIME) -> windows_core::Result<()>;
    fn GetFlags(&self, pbflags: *mut u8) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IFeedClockVectorElement {}
impl IFeedClockVectorElement_Vtbl {
    pub const fn new<Identity: IFeedClockVectorElement_Impl, const OFFSET: isize>() -> IFeedClockVectorElement_Vtbl {
        unsafe extern "system" fn GetSyncTime<Identity: IFeedClockVectorElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psynctime: *mut SYNC_TIME) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedClockVectorElement_Impl::GetSyncTime(this, core::mem::transmute_copy(&psynctime)).into()
        }
        unsafe extern "system" fn GetFlags<Identity: IFeedClockVectorElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbflags: *mut u8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedClockVectorElement_Impl::GetFlags(this, core::mem::transmute_copy(&pbflags)).into()
        }
        Self {
            base__: IClockVectorElement_Vtbl::new::<Identity, OFFSET>(),
            GetSyncTime: GetSyncTime::<Identity, OFFSET>,
            GetFlags: GetFlags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFeedClockVectorElement as windows_core::Interface>::IID || iid == &<IClockVectorElement as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IFilterKeyMap, IFilterKeyMap_Vtbl, 0xca169652_07c6_4708_a3da_6e4eba8d2297);
impl core::ops::Deref for IFilterKeyMap {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFilterKeyMap, windows_core::IUnknown);
impl IFilterKeyMap {
    pub unsafe fn GetCount(&self, pdwcount: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), pdwcount).ok()
    }
    pub unsafe fn AddFilter<P0>(&self, pisyncfilter: P0, pdwfilterkey: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISyncFilter>,
    {
        (windows_core::Interface::vtable(self).AddFilter)(windows_core::Interface::as_raw(self), pisyncfilter.param().abi(), pdwfilterkey).ok()
    }
    pub unsafe fn GetFilter(&self, dwfilterkey: u32) -> windows_core::Result<ISyncFilter> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFilter)(windows_core::Interface::as_raw(self), dwfilterkey, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Serialize(&self, pbfilterkeymap: *mut u8, pcbfilterkeymap: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Serialize)(windows_core::Interface::as_raw(self), pbfilterkeymap, pcbfilterkeymap).ok()
    }
}
#[repr(C)]
pub struct IFilterKeyMap_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub AddFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetFilter: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Serialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
}
pub trait IFilterKeyMap_Impl: Sized + windows_core::IUnknownImpl {
    fn GetCount(&self, pdwcount: *mut u32) -> windows_core::Result<()>;
    fn AddFilter(&self, pisyncfilter: Option<&ISyncFilter>, pdwfilterkey: *mut u32) -> windows_core::Result<()>;
    fn GetFilter(&self, dwfilterkey: u32) -> windows_core::Result<ISyncFilter>;
    fn Serialize(&self, pbfilterkeymap: *mut u8, pcbfilterkeymap: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IFilterKeyMap {}
impl IFilterKeyMap_Vtbl {
    pub const fn new<Identity: IFilterKeyMap_Impl, const OFFSET: isize>() -> IFilterKeyMap_Vtbl {
        unsafe extern "system" fn GetCount<Identity: IFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFilterKeyMap_Impl::GetCount(this, core::mem::transmute_copy(&pdwcount)).into()
        }
        unsafe extern "system" fn AddFilter<Identity: IFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisyncfilter: *mut core::ffi::c_void, pdwfilterkey: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFilterKeyMap_Impl::AddFilter(this, windows_core::from_raw_borrowed(&pisyncfilter), core::mem::transmute_copy(&pdwfilterkey)).into()
        }
        unsafe extern "system" fn GetFilter<Identity: IFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfilterkey: u32, ppisyncfilter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFilterKeyMap_Impl::GetFilter(this, core::mem::transmute_copy(&dwfilterkey)) {
                Ok(ok__) => {
                    ppisyncfilter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Serialize<Identity: IFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbfilterkeymap: *mut u8, pcbfilterkeymap: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFilterKeyMap_Impl::Serialize(this, core::mem::transmute_copy(&pbfilterkeymap), core::mem::transmute_copy(&pcbfilterkeymap)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            AddFilter: AddFilter::<Identity, OFFSET>,
            GetFilter: GetFilter::<Identity, OFFSET>,
            Serialize: Serialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFilterKeyMap as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IFilterRequestCallback, IFilterRequestCallback_Vtbl, 0x82df8873_6360_463a_a8a1_ede5e1a1594d);
impl core::ops::Deref for IFilterRequestCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFilterRequestCallback, windows_core::IUnknown);
impl IFilterRequestCallback {
    pub unsafe fn RequestFilter<P0>(&self, pfilter: P0, filteringtype: FILTERING_TYPE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).RequestFilter)(windows_core::Interface::as_raw(self), pfilter.param().abi(), filteringtype).ok()
    }
}
#[repr(C)]
pub struct IFilterRequestCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RequestFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, FILTERING_TYPE) -> windows_core::HRESULT,
}
pub trait IFilterRequestCallback_Impl: Sized + windows_core::IUnknownImpl {
    fn RequestFilter(&self, pfilter: Option<&windows_core::IUnknown>, filteringtype: FILTERING_TYPE) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IFilterRequestCallback {}
impl IFilterRequestCallback_Vtbl {
    pub const fn new<Identity: IFilterRequestCallback_Impl, const OFFSET: isize>() -> IFilterRequestCallback_Vtbl {
        unsafe extern "system" fn RequestFilter<Identity: IFilterRequestCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *mut core::ffi::c_void, filteringtype: FILTERING_TYPE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFilterRequestCallback_Impl::RequestFilter(this, windows_core::from_raw_borrowed(&pfilter), core::mem::transmute_copy(&filteringtype)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), RequestFilter: RequestFilter::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFilterRequestCallback as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IFilterTrackingProvider, IFilterTrackingProvider_Vtbl, 0x743383c0_fc4e_45ba_ad81_d9d84c7a24f8);
impl core::ops::Deref for IFilterTrackingProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFilterTrackingProvider, windows_core::IUnknown);
impl IFilterTrackingProvider {
    pub unsafe fn SpecifyTrackedFilters<P0>(&self, pcallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFilterTrackingRequestCallback>,
    {
        (windows_core::Interface::vtable(self).SpecifyTrackedFilters)(windows_core::Interface::as_raw(self), pcallback.param().abi()).ok()
    }
    pub unsafe fn AddTrackedFilter<P0>(&self, pfilter: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISyncFilter>,
    {
        (windows_core::Interface::vtable(self).AddTrackedFilter)(windows_core::Interface::as_raw(self), pfilter.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IFilterTrackingProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SpecifyTrackedFilters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddTrackedFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IFilterTrackingProvider_Impl: Sized + windows_core::IUnknownImpl {
    fn SpecifyTrackedFilters(&self, pcallback: Option<&IFilterTrackingRequestCallback>) -> windows_core::Result<()>;
    fn AddTrackedFilter(&self, pfilter: Option<&ISyncFilter>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IFilterTrackingProvider {}
impl IFilterTrackingProvider_Vtbl {
    pub const fn new<Identity: IFilterTrackingProvider_Impl, const OFFSET: isize>() -> IFilterTrackingProvider_Vtbl {
        unsafe extern "system" fn SpecifyTrackedFilters<Identity: IFilterTrackingProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFilterTrackingProvider_Impl::SpecifyTrackedFilters(this, windows_core::from_raw_borrowed(&pcallback)).into()
        }
        unsafe extern "system" fn AddTrackedFilter<Identity: IFilterTrackingProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFilterTrackingProvider_Impl::AddTrackedFilter(this, windows_core::from_raw_borrowed(&pfilter)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SpecifyTrackedFilters: SpecifyTrackedFilters::<Identity, OFFSET>,
            AddTrackedFilter: AddTrackedFilter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFilterTrackingProvider as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IFilterTrackingRequestCallback, IFilterTrackingRequestCallback_Vtbl, 0x713ca7bb_c858_4674_b4b6_1122436587a9);
impl core::ops::Deref for IFilterTrackingRequestCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFilterTrackingRequestCallback, windows_core::IUnknown);
impl IFilterTrackingRequestCallback {
    pub unsafe fn RequestTrackedFilter<P0>(&self, pfilter: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISyncFilter>,
    {
        (windows_core::Interface::vtable(self).RequestTrackedFilter)(windows_core::Interface::as_raw(self), pfilter.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IFilterTrackingRequestCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RequestTrackedFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IFilterTrackingRequestCallback_Impl: Sized + windows_core::IUnknownImpl {
    fn RequestTrackedFilter(&self, pfilter: Option<&ISyncFilter>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IFilterTrackingRequestCallback {}
impl IFilterTrackingRequestCallback_Vtbl {
    pub const fn new<Identity: IFilterTrackingRequestCallback_Impl, const OFFSET: isize>() -> IFilterTrackingRequestCallback_Vtbl {
        unsafe extern "system" fn RequestTrackedFilter<Identity: IFilterTrackingRequestCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFilterTrackingRequestCallback_Impl::RequestTrackedFilter(this, windows_core::from_raw_borrowed(&pfilter)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), RequestTrackedFilter: RequestTrackedFilter::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFilterTrackingRequestCallback as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IFilterTrackingSyncChangeBuilder, IFilterTrackingSyncChangeBuilder_Vtbl, 0x295024a0_70da_4c58_883c_ce2afb308d0b);
impl core::ops::Deref for IFilterTrackingSyncChangeBuilder {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFilterTrackingSyncChangeBuilder, windows_core::IUnknown);
impl IFilterTrackingSyncChangeBuilder {
    pub unsafe fn AddFilterChange(&self, dwfilterkey: u32, pfilterchange: *const SYNC_FILTER_CHANGE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddFilterChange)(windows_core::Interface::as_raw(self), dwfilterkey, pfilterchange).ok()
    }
    pub unsafe fn SetAllChangeUnitsPresentFlag(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAllChangeUnitsPresentFlag)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IFilterTrackingSyncChangeBuilder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddFilterChange: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const SYNC_FILTER_CHANGE) -> windows_core::HRESULT,
    pub SetAllChangeUnitsPresentFlag: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IFilterTrackingSyncChangeBuilder_Impl: Sized + windows_core::IUnknownImpl {
    fn AddFilterChange(&self, dwfilterkey: u32, pfilterchange: *const SYNC_FILTER_CHANGE) -> windows_core::Result<()>;
    fn SetAllChangeUnitsPresentFlag(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IFilterTrackingSyncChangeBuilder {}
impl IFilterTrackingSyncChangeBuilder_Vtbl {
    pub const fn new<Identity: IFilterTrackingSyncChangeBuilder_Impl, const OFFSET: isize>() -> IFilterTrackingSyncChangeBuilder_Vtbl {
        unsafe extern "system" fn AddFilterChange<Identity: IFilterTrackingSyncChangeBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfilterkey: u32, pfilterchange: *const SYNC_FILTER_CHANGE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFilterTrackingSyncChangeBuilder_Impl::AddFilterChange(this, core::mem::transmute_copy(&dwfilterkey), core::mem::transmute_copy(&pfilterchange)).into()
        }
        unsafe extern "system" fn SetAllChangeUnitsPresentFlag<Identity: IFilterTrackingSyncChangeBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFilterTrackingSyncChangeBuilder_Impl::SetAllChangeUnitsPresentFlag(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddFilterChange: AddFilterChange::<Identity, OFFSET>,
            SetAllChangeUnitsPresentFlag: SetAllChangeUnitsPresentFlag::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFilterTrackingSyncChangeBuilder as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IForgottenKnowledge, IForgottenKnowledge_Vtbl, 0x456e0f96_6036_452b_9f9d_bcc4b4a85db2);
impl core::ops::Deref for IForgottenKnowledge {
    type Target = ISyncKnowledge;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IForgottenKnowledge, windows_core::IUnknown, ISyncKnowledge);
impl IForgottenKnowledge {
    pub unsafe fn ForgetToVersion<P0>(&self, pknowledge: P0, pversion: *const SYNC_VERSION) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISyncKnowledge>,
    {
        (windows_core::Interface::vtable(self).ForgetToVersion)(windows_core::Interface::as_raw(self), pknowledge.param().abi(), pversion).ok()
    }
}
#[repr(C)]
pub struct IForgottenKnowledge_Vtbl {
    pub base__: ISyncKnowledge_Vtbl,
    pub ForgetToVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const SYNC_VERSION) -> windows_core::HRESULT,
}
pub trait IForgottenKnowledge_Impl: Sized + ISyncKnowledge_Impl {
    fn ForgetToVersion(&self, pknowledge: Option<&ISyncKnowledge>, pversion: *const SYNC_VERSION) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IForgottenKnowledge {}
impl IForgottenKnowledge_Vtbl {
    pub const fn new<Identity: IForgottenKnowledge_Impl, const OFFSET: isize>() -> IForgottenKnowledge_Vtbl {
        unsafe extern "system" fn ForgetToVersion<Identity: IForgottenKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pknowledge: *mut core::ffi::c_void, pversion: *const SYNC_VERSION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IForgottenKnowledge_Impl::ForgetToVersion(this, windows_core::from_raw_borrowed(&pknowledge), core::mem::transmute_copy(&pversion)).into()
        }
        Self { base__: ISyncKnowledge_Vtbl::new::<Identity, OFFSET>(), ForgetToVersion: ForgetToVersion::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IForgottenKnowledge as windows_core::Interface>::IID || iid == &<ISyncKnowledge as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IKnowledgeSyncProvider, IKnowledgeSyncProvider_Vtbl, 0x43434a49_8da4_47f2_8172_ad7b8b024978);
impl core::ops::Deref for IKnowledgeSyncProvider {
    type Target = ISyncProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IKnowledgeSyncProvider, windows_core::IUnknown, ISyncProvider);
impl IKnowledgeSyncProvider {
    pub unsafe fn BeginSession<P0>(&self, role: SYNC_PROVIDER_ROLE, psessionstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISyncSessionState>,
    {
        (windows_core::Interface::vtable(self).BeginSession)(windows_core::Interface::as_raw(self), role, psessionstate.param().abi()).ok()
    }
    pub unsafe fn GetSyncBatchParameters(&self, ppsyncknowledge: *mut Option<ISyncKnowledge>, pdwrequestedbatchsize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSyncBatchParameters)(windows_core::Interface::as_raw(self), core::mem::transmute(ppsyncknowledge), pdwrequestedbatchsize).ok()
    }
    pub unsafe fn GetChangeBatch<P0>(&self, dwbatchsize: u32, psyncknowledge: P0, ppsyncchangebatch: *mut Option<ISyncChangeBatch>, ppunkdataretriever: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISyncKnowledge>,
    {
        (windows_core::Interface::vtable(self).GetChangeBatch)(windows_core::Interface::as_raw(self), dwbatchsize, psyncknowledge.param().abi(), core::mem::transmute(ppsyncchangebatch), core::mem::transmute(ppunkdataretriever)).ok()
    }
    pub unsafe fn GetFullEnumerationChangeBatch<P0>(&self, dwbatchsize: u32, pblowerenumerationbound: *const u8, psyncknowledge: P0, ppsyncchangebatch: *mut Option<ISyncFullEnumerationChangeBatch>, ppunkdataretriever: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISyncKnowledge>,
    {
        (windows_core::Interface::vtable(self).GetFullEnumerationChangeBatch)(windows_core::Interface::as_raw(self), dwbatchsize, pblowerenumerationbound, psyncknowledge.param().abi(), core::mem::transmute(ppsyncchangebatch), core::mem::transmute(ppunkdataretriever)).ok()
    }
    pub unsafe fn ProcessChangeBatch<P0, P1, P2>(&self, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: P0, punkdataretriever: P1, pcallback: P2, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISyncChangeBatch>,
        P1: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<ISyncCallback>,
    {
        (windows_core::Interface::vtable(self).ProcessChangeBatch)(windows_core::Interface::as_raw(self), resolutionpolicy, psourcechangebatch.param().abi(), punkdataretriever.param().abi(), pcallback.param().abi(), psyncsessionstatistics).ok()
    }
    pub unsafe fn ProcessFullEnumerationChangeBatch<P0, P1, P2>(&self, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: P0, punkdataretriever: P1, pcallback: P2, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISyncFullEnumerationChangeBatch>,
        P1: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<ISyncCallback>,
    {
        (windows_core::Interface::vtable(self).ProcessFullEnumerationChangeBatch)(windows_core::Interface::as_raw(self), resolutionpolicy, psourcechangebatch.param().abi(), punkdataretriever.param().abi(), pcallback.param().abi(), psyncsessionstatistics).ok()
    }
    pub unsafe fn EndSession<P0>(&self, psessionstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISyncSessionState>,
    {
        (windows_core::Interface::vtable(self).EndSession)(windows_core::Interface::as_raw(self), psessionstate.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IKnowledgeSyncProvider_Vtbl {
    pub base__: ISyncProvider_Vtbl,
    pub BeginSession: unsafe extern "system" fn(*mut core::ffi::c_void, SYNC_PROVIDER_ROLE, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSyncBatchParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetChangeBatch: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFullEnumerationChangeBatch: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProcessChangeBatch: unsafe extern "system" fn(*mut core::ffi::c_void, CONFLICT_RESOLUTION_POLICY, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut SYNC_SESSION_STATISTICS) -> windows_core::HRESULT,
    pub ProcessFullEnumerationChangeBatch: unsafe extern "system" fn(*mut core::ffi::c_void, CONFLICT_RESOLUTION_POLICY, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut SYNC_SESSION_STATISTICS) -> windows_core::HRESULT,
    pub EndSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IKnowledgeSyncProvider_Impl: Sized + ISyncProvider_Impl {
    fn BeginSession(&self, role: SYNC_PROVIDER_ROLE, psessionstate: Option<&ISyncSessionState>) -> windows_core::Result<()>;
    fn GetSyncBatchParameters(&self, ppsyncknowledge: *mut Option<ISyncKnowledge>, pdwrequestedbatchsize: *mut u32) -> windows_core::Result<()>;
    fn GetChangeBatch(&self, dwbatchsize: u32, psyncknowledge: Option<&ISyncKnowledge>, ppsyncchangebatch: *mut Option<ISyncChangeBatch>, ppunkdataretriever: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetFullEnumerationChangeBatch(&self, dwbatchsize: u32, pblowerenumerationbound: *const u8, psyncknowledge: Option<&ISyncKnowledge>, ppsyncchangebatch: *mut Option<ISyncFullEnumerationChangeBatch>, ppunkdataretriever: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn ProcessChangeBatch(&self, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: Option<&ISyncChangeBatch>, punkdataretriever: Option<&windows_core::IUnknown>, pcallback: Option<&ISyncCallback>, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> windows_core::Result<()>;
    fn ProcessFullEnumerationChangeBatch(&self, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: Option<&ISyncFullEnumerationChangeBatch>, punkdataretriever: Option<&windows_core::IUnknown>, pcallback: Option<&ISyncCallback>, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> windows_core::Result<()>;
    fn EndSession(&self, psessionstate: Option<&ISyncSessionState>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IKnowledgeSyncProvider {}
impl IKnowledgeSyncProvider_Vtbl {
    pub const fn new<Identity: IKnowledgeSyncProvider_Impl, const OFFSET: isize>() -> IKnowledgeSyncProvider_Vtbl {
        unsafe extern "system" fn BeginSession<Identity: IKnowledgeSyncProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, role: SYNC_PROVIDER_ROLE, psessionstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKnowledgeSyncProvider_Impl::BeginSession(this, core::mem::transmute_copy(&role), windows_core::from_raw_borrowed(&psessionstate)).into()
        }
        unsafe extern "system" fn GetSyncBatchParameters<Identity: IKnowledgeSyncProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsyncknowledge: *mut *mut core::ffi::c_void, pdwrequestedbatchsize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKnowledgeSyncProvider_Impl::GetSyncBatchParameters(this, core::mem::transmute_copy(&ppsyncknowledge), core::mem::transmute_copy(&pdwrequestedbatchsize)).into()
        }
        unsafe extern "system" fn GetChangeBatch<Identity: IKnowledgeSyncProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwbatchsize: u32, psyncknowledge: *mut core::ffi::c_void, ppsyncchangebatch: *mut *mut core::ffi::c_void, ppunkdataretriever: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKnowledgeSyncProvider_Impl::GetChangeBatch(this, core::mem::transmute_copy(&dwbatchsize), windows_core::from_raw_borrowed(&psyncknowledge), core::mem::transmute_copy(&ppsyncchangebatch), core::mem::transmute_copy(&ppunkdataretriever)).into()
        }
        unsafe extern "system" fn GetFullEnumerationChangeBatch<Identity: IKnowledgeSyncProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwbatchsize: u32, pblowerenumerationbound: *const u8, psyncknowledge: *mut core::ffi::c_void, ppsyncchangebatch: *mut *mut core::ffi::c_void, ppunkdataretriever: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKnowledgeSyncProvider_Impl::GetFullEnumerationChangeBatch(this, core::mem::transmute_copy(&dwbatchsize), core::mem::transmute_copy(&pblowerenumerationbound), windows_core::from_raw_borrowed(&psyncknowledge), core::mem::transmute_copy(&ppsyncchangebatch), core::mem::transmute_copy(&ppunkdataretriever)).into()
        }
        unsafe extern "system" fn ProcessChangeBatch<Identity: IKnowledgeSyncProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: *mut core::ffi::c_void, punkdataretriever: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKnowledgeSyncProvider_Impl::ProcessChangeBatch(this, core::mem::transmute_copy(&resolutionpolicy), windows_core::from_raw_borrowed(&psourcechangebatch), windows_core::from_raw_borrowed(&punkdataretriever), windows_core::from_raw_borrowed(&pcallback), core::mem::transmute_copy(&psyncsessionstatistics)).into()
        }
        unsafe extern "system" fn ProcessFullEnumerationChangeBatch<Identity: IKnowledgeSyncProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: *mut core::ffi::c_void, punkdataretriever: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKnowledgeSyncProvider_Impl::ProcessFullEnumerationChangeBatch(this, core::mem::transmute_copy(&resolutionpolicy), windows_core::from_raw_borrowed(&psourcechangebatch), windows_core::from_raw_borrowed(&punkdataretriever), windows_core::from_raw_borrowed(&pcallback), core::mem::transmute_copy(&psyncsessionstatistics)).into()
        }
        unsafe extern "system" fn EndSession<Identity: IKnowledgeSyncProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psessionstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKnowledgeSyncProvider_Impl::EndSession(this, windows_core::from_raw_borrowed(&psessionstate)).into()
        }
        Self {
            base__: ISyncProvider_Vtbl::new::<Identity, OFFSET>(),
            BeginSession: BeginSession::<Identity, OFFSET>,
            GetSyncBatchParameters: GetSyncBatchParameters::<Identity, OFFSET>,
            GetChangeBatch: GetChangeBatch::<Identity, OFFSET>,
            GetFullEnumerationChangeBatch: GetFullEnumerationChangeBatch::<Identity, OFFSET>,
            ProcessChangeBatch: ProcessChangeBatch::<Identity, OFFSET>,
            ProcessFullEnumerationChangeBatch: ProcessFullEnumerationChangeBatch::<Identity, OFFSET>,
            EndSession: EndSession::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKnowledgeSyncProvider as windows_core::Interface>::IID || iid == &<ISyncProvider as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ILoadChangeContext, ILoadChangeContext_Vtbl, 0x44a4aaca_ec39_46d5_b5c9_d633c0ee67e2);
impl core::ops::Deref for ILoadChangeContext {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ILoadChangeContext, windows_core::IUnknown);
impl ILoadChangeContext {
    pub unsafe fn GetSyncChange(&self) -> windows_core::Result<ISyncChange> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSyncChange)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetRecoverableErrorOnChange<P0>(&self, hrerror: windows_core::HRESULT, perrordata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRecoverableErrorData>,
    {
        (windows_core::Interface::vtable(self).SetRecoverableErrorOnChange)(windows_core::Interface::as_raw(self), hrerror, perrordata.param().abi()).ok()
    }
    pub unsafe fn SetRecoverableErrorOnChangeUnit<P0, P1>(&self, hrerror: windows_core::HRESULT, pchangeunit: P0, perrordata: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISyncChangeUnit>,
        P1: windows_core::Param<IRecoverableErrorData>,
    {
        (windows_core::Interface::vtable(self).SetRecoverableErrorOnChangeUnit)(windows_core::Interface::as_raw(self), hrerror, pchangeunit.param().abi(), perrordata.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ILoadChangeContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSyncChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRecoverableErrorOnChange: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRecoverableErrorOnChangeUnit: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ILoadChangeContext_Impl: Sized + windows_core::IUnknownImpl {
    fn GetSyncChange(&self) -> windows_core::Result<ISyncChange>;
    fn SetRecoverableErrorOnChange(&self, hrerror: windows_core::HRESULT, perrordata: Option<&IRecoverableErrorData>) -> windows_core::Result<()>;
    fn SetRecoverableErrorOnChangeUnit(&self, hrerror: windows_core::HRESULT, pchangeunit: Option<&ISyncChangeUnit>, perrordata: Option<&IRecoverableErrorData>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ILoadChangeContext {}
impl ILoadChangeContext_Vtbl {
    pub const fn new<Identity: ILoadChangeContext_Impl, const OFFSET: isize>() -> ILoadChangeContext_Vtbl {
        unsafe extern "system" fn GetSyncChange<Identity: ILoadChangeContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsyncchange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILoadChangeContext_Impl::GetSyncChange(this) {
                Ok(ok__) => {
                    ppsyncchange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecoverableErrorOnChange<Identity: ILoadChangeContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrerror: windows_core::HRESULT, perrordata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILoadChangeContext_Impl::SetRecoverableErrorOnChange(this, core::mem::transmute_copy(&hrerror), windows_core::from_raw_borrowed(&perrordata)).into()
        }
        unsafe extern "system" fn SetRecoverableErrorOnChangeUnit<Identity: ILoadChangeContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrerror: windows_core::HRESULT, pchangeunit: *mut core::ffi::c_void, perrordata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILoadChangeContext_Impl::SetRecoverableErrorOnChangeUnit(this, core::mem::transmute_copy(&hrerror), windows_core::from_raw_borrowed(&pchangeunit), windows_core::from_raw_borrowed(&perrordata)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSyncChange: GetSyncChange::<Identity, OFFSET>,
            SetRecoverableErrorOnChange: SetRecoverableErrorOnChange::<Identity, OFFSET>,
            SetRecoverableErrorOnChangeUnit: SetRecoverableErrorOnChangeUnit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILoadChangeContext as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IProviderConverter, IProviderConverter_Vtbl, 0x809b7276_98cf_4957_93a5_0ebdd3dddffd);
impl core::ops::Deref for IProviderConverter {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IProviderConverter, windows_core::IUnknown);
impl IProviderConverter {
    pub unsafe fn Initialize<P0>(&self, pisyncprovider: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISyncProvider>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pisyncprovider.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IProviderConverter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IProviderConverter_Impl: Sized + windows_core::IUnknownImpl {
    fn Initialize(&self, pisyncprovider: Option<&ISyncProvider>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IProviderConverter {}
impl IProviderConverter_Vtbl {
    pub const fn new<Identity: IProviderConverter_Impl, const OFFSET: isize>() -> IProviderConverter_Vtbl {
        unsafe extern "system" fn Initialize<Identity: IProviderConverter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisyncprovider: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IProviderConverter_Impl::Initialize(this, windows_core::from_raw_borrowed(&pisyncprovider)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProviderConverter as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IRangeException, IRangeException_Vtbl, 0x75ae8777_6848_49f7_956c_a3a92f5096e8);
impl core::ops::Deref for IRangeException {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRangeException, windows_core::IUnknown);
impl IRangeException {
    pub unsafe fn GetClosedRangeStart(&self, pbclosedrangestart: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetClosedRangeStart)(windows_core::Interface::as_raw(self), pbclosedrangestart, pcbidsize).ok()
    }
    pub unsafe fn GetClosedRangeEnd(&self, pbclosedrangeend: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetClosedRangeEnd)(windows_core::Interface::as_raw(self), pbclosedrangeend, pcbidsize).ok()
    }
    pub unsafe fn GetClockVector(&self, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetClockVector)(windows_core::Interface::as_raw(self), riid, ppunk).ok()
    }
}
#[repr(C)]
pub struct IRangeException_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetClosedRangeStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetClosedRangeEnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetClockVector: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRangeException_Impl: Sized + windows_core::IUnknownImpl {
    fn GetClosedRangeStart(&self, pbclosedrangestart: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()>;
    fn GetClosedRangeEnd(&self, pbclosedrangeend: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()>;
    fn GetClockVector(&self, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRangeException {}
impl IRangeException_Vtbl {
    pub const fn new<Identity: IRangeException_Impl, const OFFSET: isize>() -> IRangeException_Vtbl {
        unsafe extern "system" fn GetClosedRangeStart<Identity: IRangeException_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbclosedrangestart: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRangeException_Impl::GetClosedRangeStart(this, core::mem::transmute_copy(&pbclosedrangestart), core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetClosedRangeEnd<Identity: IRangeException_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbclosedrangeend: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRangeException_Impl::GetClosedRangeEnd(this, core::mem::transmute_copy(&pbclosedrangeend), core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetClockVector<Identity: IRangeException_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRangeException_Impl::GetClockVector(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClosedRangeStart: GetClosedRangeStart::<Identity, OFFSET>,
            GetClosedRangeEnd: GetClosedRangeEnd::<Identity, OFFSET>,
            GetClockVector: GetClockVector::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRangeException as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IRecoverableError, IRecoverableError_Vtbl, 0x0f5625e8_0a7b_45ee_9637_1ce13645909e);
impl core::ops::Deref for IRecoverableError {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRecoverableError, windows_core::IUnknown);
impl IRecoverableError {
    pub unsafe fn GetStage(&self, pstage: *mut SYNC_PROGRESS_STAGE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStage)(windows_core::Interface::as_raw(self), pstage).ok()
    }
    pub unsafe fn GetProvider(&self, pproviderrole: *mut SYNC_PROVIDER_ROLE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProvider)(windows_core::Interface::as_raw(self), pproviderrole).ok()
    }
    pub unsafe fn GetChangeWithRecoverableError(&self) -> windows_core::Result<ISyncChange> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetChangeWithRecoverableError)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetRecoverableErrorDataForChange(&self, phrerror: *mut windows_core::HRESULT, pperrordata: *mut Option<IRecoverableErrorData>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRecoverableErrorDataForChange)(windows_core::Interface::as_raw(self), phrerror, core::mem::transmute(pperrordata)).ok()
    }
    pub unsafe fn GetRecoverableErrorDataForChangeUnit<P0>(&self, pchangeunit: P0, phrerror: *mut windows_core::HRESULT, pperrordata: *mut Option<IRecoverableErrorData>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISyncChangeUnit>,
    {
        (windows_core::Interface::vtable(self).GetRecoverableErrorDataForChangeUnit)(windows_core::Interface::as_raw(self), pchangeunit.param().abi(), phrerror, core::mem::transmute(pperrordata)).ok()
    }
}
#[repr(C)]
pub struct IRecoverableError_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SYNC_PROGRESS_STAGE) -> windows_core::HRESULT,
    pub GetProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SYNC_PROVIDER_ROLE) -> windows_core::HRESULT,
    pub GetChangeWithRecoverableError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRecoverableErrorDataForChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRecoverableErrorDataForChangeUnit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::HRESULT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRecoverableError_Impl: Sized + windows_core::IUnknownImpl {
    fn GetStage(&self, pstage: *mut SYNC_PROGRESS_STAGE) -> windows_core::Result<()>;
    fn GetProvider(&self, pproviderrole: *mut SYNC_PROVIDER_ROLE) -> windows_core::Result<()>;
    fn GetChangeWithRecoverableError(&self) -> windows_core::Result<ISyncChange>;
    fn GetRecoverableErrorDataForChange(&self, phrerror: *mut windows_core::HRESULT, pperrordata: *mut Option<IRecoverableErrorData>) -> windows_core::Result<()>;
    fn GetRecoverableErrorDataForChangeUnit(&self, pchangeunit: Option<&ISyncChangeUnit>, phrerror: *mut windows_core::HRESULT, pperrordata: *mut Option<IRecoverableErrorData>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRecoverableError {}
impl IRecoverableError_Vtbl {
    pub const fn new<Identity: IRecoverableError_Impl, const OFFSET: isize>() -> IRecoverableError_Vtbl {
        unsafe extern "system" fn GetStage<Identity: IRecoverableError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstage: *mut SYNC_PROGRESS_STAGE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRecoverableError_Impl::GetStage(this, core::mem::transmute_copy(&pstage)).into()
        }
        unsafe extern "system" fn GetProvider<Identity: IRecoverableError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproviderrole: *mut SYNC_PROVIDER_ROLE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRecoverableError_Impl::GetProvider(this, core::mem::transmute_copy(&pproviderrole)).into()
        }
        unsafe extern "system" fn GetChangeWithRecoverableError<Identity: IRecoverableError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppchangewithrecoverableerror: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRecoverableError_Impl::GetChangeWithRecoverableError(this) {
                Ok(ok__) => {
                    ppchangewithrecoverableerror.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecoverableErrorDataForChange<Identity: IRecoverableError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrerror: *mut windows_core::HRESULT, pperrordata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRecoverableError_Impl::GetRecoverableErrorDataForChange(this, core::mem::transmute_copy(&phrerror), core::mem::transmute_copy(&pperrordata)).into()
        }
        unsafe extern "system" fn GetRecoverableErrorDataForChangeUnit<Identity: IRecoverableError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchangeunit: *mut core::ffi::c_void, phrerror: *mut windows_core::HRESULT, pperrordata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRecoverableError_Impl::GetRecoverableErrorDataForChangeUnit(this, windows_core::from_raw_borrowed(&pchangeunit), core::mem::transmute_copy(&phrerror), core::mem::transmute_copy(&pperrordata)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStage: GetStage::<Identity, OFFSET>,
            GetProvider: GetProvider::<Identity, OFFSET>,
            GetChangeWithRecoverableError: GetChangeWithRecoverableError::<Identity, OFFSET>,
            GetRecoverableErrorDataForChange: GetRecoverableErrorDataForChange::<Identity, OFFSET>,
            GetRecoverableErrorDataForChangeUnit: GetRecoverableErrorDataForChangeUnit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRecoverableError as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IRecoverableErrorData, IRecoverableErrorData_Vtbl, 0xb37c4a0a_4b7d_4c2d_9711_3b00d119b1c8);
impl core::ops::Deref for IRecoverableErrorData {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRecoverableErrorData, windows_core::IUnknown);
impl IRecoverableErrorData {
    pub unsafe fn Initialize<P0, P1>(&self, pcszitemdisplayname: P0, pcszerrordescription: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pcszitemdisplayname.param().abi(), pcszerrordescription.param().abi()).ok()
    }
    pub unsafe fn GetItemDisplayName<P0>(&self, pszitemdisplayname: P0, pcchitemdisplayname: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetItemDisplayName)(windows_core::Interface::as_raw(self), pszitemdisplayname.param().abi(), pcchitemdisplayname).ok()
    }
    pub unsafe fn GetErrorDescription<P0>(&self, pszerrordescription: P0, pccherrordescription: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetErrorDescription)(windows_core::Interface::as_raw(self), pszerrordescription.param().abi(), pccherrordescription).ok()
    }
}
#[repr(C)]
pub struct IRecoverableErrorData_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetItemDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetErrorDescription: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
}
pub trait IRecoverableErrorData_Impl: Sized + windows_core::IUnknownImpl {
    fn Initialize(&self, pcszitemdisplayname: &windows_core::PCWSTR, pcszerrordescription: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetItemDisplayName(&self, pszitemdisplayname: &windows_core::PCWSTR, pcchitemdisplayname: *mut u32) -> windows_core::Result<()>;
    fn GetErrorDescription(&self, pszerrordescription: &windows_core::PCWSTR, pccherrordescription: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRecoverableErrorData {}
impl IRecoverableErrorData_Vtbl {
    pub const fn new<Identity: IRecoverableErrorData_Impl, const OFFSET: isize>() -> IRecoverableErrorData_Vtbl {
        unsafe extern "system" fn Initialize<Identity: IRecoverableErrorData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcszitemdisplayname: windows_core::PCWSTR, pcszerrordescription: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRecoverableErrorData_Impl::Initialize(this, core::mem::transmute(&pcszitemdisplayname), core::mem::transmute(&pcszerrordescription)).into()
        }
        unsafe extern "system" fn GetItemDisplayName<Identity: IRecoverableErrorData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszitemdisplayname: windows_core::PCWSTR, pcchitemdisplayname: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRecoverableErrorData_Impl::GetItemDisplayName(this, core::mem::transmute(&pszitemdisplayname), core::mem::transmute_copy(&pcchitemdisplayname)).into()
        }
        unsafe extern "system" fn GetErrorDescription<Identity: IRecoverableErrorData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszerrordescription: windows_core::PCWSTR, pccherrordescription: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRecoverableErrorData_Impl::GetErrorDescription(this, core::mem::transmute(&pszerrordescription), core::mem::transmute_copy(&pccherrordescription)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            GetItemDisplayName: GetItemDisplayName::<Identity, OFFSET>,
            GetErrorDescription: GetErrorDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRecoverableErrorData as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IRegisteredSyncProvider, IRegisteredSyncProvider_Vtbl, 0x913bcf76_47c1_40b5_a896_5e8a9c414c14);
impl core::ops::Deref for IRegisteredSyncProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRegisteredSyncProvider, windows_core::IUnknown);
impl IRegisteredSyncProvider {
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Init<P0>(&self, pguidinstanceid: *const windows_core::GUID, pguidcontenttype: *const windows_core::GUID, pcontextpropertystore: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
    {
        (windows_core::Interface::vtable(self).Init)(windows_core::Interface::as_raw(self), pguidinstanceid, pguidcontenttype, pcontextpropertystore.param().abi()).ok()
    }
    pub unsafe fn GetInstanceId(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInstanceId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IRegisteredSyncProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Init: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Init: usize,
    pub GetInstanceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IRegisteredSyncProvider_Impl: Sized + windows_core::IUnknownImpl {
    fn Init(&self, pguidinstanceid: *const windows_core::GUID, pguidcontenttype: *const windows_core::GUID, pcontextpropertystore: Option<&super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> windows_core::Result<()>;
    fn GetInstanceId(&self) -> windows_core::Result<windows_core::GUID>;
    fn Reset(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for IRegisteredSyncProvider {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IRegisteredSyncProvider_Vtbl {
    pub const fn new<Identity: IRegisteredSyncProvider_Impl, const OFFSET: isize>() -> IRegisteredSyncProvider_Vtbl {
        unsafe extern "system" fn Init<Identity: IRegisteredSyncProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidinstanceid: *const windows_core::GUID, pguidcontenttype: *const windows_core::GUID, pcontextpropertystore: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRegisteredSyncProvider_Impl::Init(this, core::mem::transmute_copy(&pguidinstanceid), core::mem::transmute_copy(&pguidcontenttype), windows_core::from_raw_borrowed(&pcontextpropertystore)).into()
        }
        unsafe extern "system" fn GetInstanceId<Identity: IRegisteredSyncProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidinstanceid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRegisteredSyncProvider_Impl::GetInstanceId(this) {
                Ok(ok__) => {
                    pguidinstanceid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: IRegisteredSyncProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRegisteredSyncProvider_Impl::Reset(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, OFFSET>,
            GetInstanceId: GetInstanceId::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRegisteredSyncProvider as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IReplicaKeyMap, IReplicaKeyMap_Vtbl, 0x2209f4fc_fd10_4ff0_84a8_f0a1982e440e);
impl core::ops::Deref for IReplicaKeyMap {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IReplicaKeyMap, windows_core::IUnknown);
impl IReplicaKeyMap {
    pub unsafe fn LookupReplicaKey(&self, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LookupReplicaKey)(windows_core::Interface::as_raw(self), pbreplicaid, pdwreplicakey).ok()
    }
    pub unsafe fn LookupReplicaId(&self, dwreplicakey: u32, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LookupReplicaId)(windows_core::Interface::as_raw(self), dwreplicakey, pbreplicaid, pcbidsize).ok()
    }
    pub unsafe fn Serialize(&self, pbreplicakeymap: *mut u8, pcbreplicakeymap: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Serialize)(windows_core::Interface::as_raw(self), pbreplicakeymap, pcbreplicakeymap).ok()
    }
}
#[repr(C)]
pub struct IReplicaKeyMap_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub LookupReplicaKey: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *mut u32) -> windows_core::HRESULT,
    pub LookupReplicaId: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub Serialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
}
pub trait IReplicaKeyMap_Impl: Sized + windows_core::IUnknownImpl {
    fn LookupReplicaKey(&self, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> windows_core::Result<()>;
    fn LookupReplicaId(&self, dwreplicakey: u32, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()>;
    fn Serialize(&self, pbreplicakeymap: *mut u8, pcbreplicakeymap: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IReplicaKeyMap {}
impl IReplicaKeyMap_Vtbl {
    pub const fn new<Identity: IReplicaKeyMap_Impl, const OFFSET: isize>() -> IReplicaKeyMap_Vtbl {
        unsafe extern "system" fn LookupReplicaKey<Identity: IReplicaKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReplicaKeyMap_Impl::LookupReplicaKey(this, core::mem::transmute_copy(&pbreplicaid), core::mem::transmute_copy(&pdwreplicakey)).into()
        }
        unsafe extern "system" fn LookupReplicaId<Identity: IReplicaKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwreplicakey: u32, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReplicaKeyMap_Impl::LookupReplicaId(this, core::mem::transmute_copy(&dwreplicakey), core::mem::transmute_copy(&pbreplicaid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn Serialize<Identity: IReplicaKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbreplicakeymap: *mut u8, pcbreplicakeymap: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReplicaKeyMap_Impl::Serialize(this, core::mem::transmute_copy(&pbreplicakeymap), core::mem::transmute_copy(&pcbreplicakeymap)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LookupReplicaKey: LookupReplicaKey::<Identity, OFFSET>,
            LookupReplicaId: LookupReplicaId::<Identity, OFFSET>,
            Serialize: Serialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IReplicaKeyMap as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IRequestFilteredSync, IRequestFilteredSync_Vtbl, 0x2e020184_6d18_46a7_a32a_da4aeb06696c);
impl core::ops::Deref for IRequestFilteredSync {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRequestFilteredSync, windows_core::IUnknown);
impl IRequestFilteredSync {
    pub unsafe fn SpecifyFilter<P0>(&self, pcallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFilterRequestCallback>,
    {
        (windows_core::Interface::vtable(self).SpecifyFilter)(windows_core::Interface::as_raw(self), pcallback.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IRequestFilteredSync_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SpecifyFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRequestFilteredSync_Impl: Sized + windows_core::IUnknownImpl {
    fn SpecifyFilter(&self, pcallback: Option<&IFilterRequestCallback>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRequestFilteredSync {}
impl IRequestFilteredSync_Vtbl {
    pub const fn new<Identity: IRequestFilteredSync_Impl, const OFFSET: isize>() -> IRequestFilteredSync_Vtbl {
        unsafe extern "system" fn SpecifyFilter<Identity: IRequestFilteredSync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRequestFilteredSync_Impl::SpecifyFilter(this, windows_core::from_raw_borrowed(&pcallback)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SpecifyFilter: SpecifyFilter::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRequestFilteredSync as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISingleItemException, ISingleItemException_Vtbl, 0x892fb9b0_7c55_4a18_9316_fdf449569b64);
impl core::ops::Deref for ISingleItemException {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISingleItemException, windows_core::IUnknown);
impl ISingleItemException {
    pub unsafe fn GetItemId(&self, pbitemid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetItemId)(windows_core::Interface::as_raw(self), pbitemid, pcbidsize).ok()
    }
    pub unsafe fn GetClockVector(&self, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetClockVector)(windows_core::Interface::as_raw(self), riid, ppunk).ok()
    }
}
#[repr(C)]
pub struct ISingleItemException_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetItemId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetClockVector: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISingleItemException_Impl: Sized + windows_core::IUnknownImpl {
    fn GetItemId(&self, pbitemid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()>;
    fn GetClockVector(&self, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISingleItemException {}
impl ISingleItemException_Vtbl {
    pub const fn new<Identity: ISingleItemException_Impl, const OFFSET: isize>() -> ISingleItemException_Vtbl {
        unsafe extern "system" fn GetItemId<Identity: ISingleItemException_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitemid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISingleItemException_Impl::GetItemId(this, core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetClockVector<Identity: ISingleItemException_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISingleItemException_Impl::GetClockVector(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetItemId: GetItemId::<Identity, OFFSET>,
            GetClockVector: GetClockVector::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISingleItemException as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISupportFilteredSync, ISupportFilteredSync_Vtbl, 0x3d128ded_d555_4e0d_bf4b_fb213a8a9302);
impl core::ops::Deref for ISupportFilteredSync {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISupportFilteredSync, windows_core::IUnknown);
impl ISupportFilteredSync {
    pub unsafe fn AddFilter<P0>(&self, pfilter: P0, filteringtype: FILTERING_TYPE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).AddFilter)(windows_core::Interface::as_raw(self), pfilter.param().abi(), filteringtype).ok()
    }
}
#[repr(C)]
pub struct ISupportFilteredSync_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, FILTERING_TYPE) -> windows_core::HRESULT,
}
pub trait ISupportFilteredSync_Impl: Sized + windows_core::IUnknownImpl {
    fn AddFilter(&self, pfilter: Option<&windows_core::IUnknown>, filteringtype: FILTERING_TYPE) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISupportFilteredSync {}
impl ISupportFilteredSync_Vtbl {
    pub const fn new<Identity: ISupportFilteredSync_Impl, const OFFSET: isize>() -> ISupportFilteredSync_Vtbl {
        unsafe extern "system" fn AddFilter<Identity: ISupportFilteredSync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *mut core::ffi::c_void, filteringtype: FILTERING_TYPE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISupportFilteredSync_Impl::AddFilter(this, windows_core::from_raw_borrowed(&pfilter), core::mem::transmute_copy(&filteringtype)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddFilter: AddFilter::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISupportFilteredSync as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISupportLastWriteTime, ISupportLastWriteTime_Vtbl, 0xeadf816f_d0bd_43ca_8f40_5acdc6c06f7a);
impl core::ops::Deref for ISupportLastWriteTime {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISupportLastWriteTime, windows_core::IUnknown);
impl ISupportLastWriteTime {
    pub unsafe fn GetItemChangeTime(&self, pbitemid: *const u8, pulltimestamp: *mut u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetItemChangeTime)(windows_core::Interface::as_raw(self), pbitemid, pulltimestamp).ok()
    }
    pub unsafe fn GetChangeUnitChangeTime(&self, pbitemid: *const u8, pbchangeunitid: *const u8, pulltimestamp: *mut u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetChangeUnitChangeTime)(windows_core::Interface::as_raw(self), pbitemid, pbchangeunitid, pulltimestamp).ok()
    }
}
#[repr(C)]
pub struct ISupportLastWriteTime_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetItemChangeTime: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *mut u64) -> windows_core::HRESULT,
    pub GetChangeUnitChangeTime: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *const u8, *mut u64) -> windows_core::HRESULT,
}
pub trait ISupportLastWriteTime_Impl: Sized + windows_core::IUnknownImpl {
    fn GetItemChangeTime(&self, pbitemid: *const u8, pulltimestamp: *mut u64) -> windows_core::Result<()>;
    fn GetChangeUnitChangeTime(&self, pbitemid: *const u8, pbchangeunitid: *const u8, pulltimestamp: *mut u64) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISupportLastWriteTime {}
impl ISupportLastWriteTime_Vtbl {
    pub const fn new<Identity: ISupportLastWriteTime_Impl, const OFFSET: isize>() -> ISupportLastWriteTime_Vtbl {
        unsafe extern "system" fn GetItemChangeTime<Identity: ISupportLastWriteTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitemid: *const u8, pulltimestamp: *mut u64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISupportLastWriteTime_Impl::GetItemChangeTime(this, core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pulltimestamp)).into()
        }
        unsafe extern "system" fn GetChangeUnitChangeTime<Identity: ISupportLastWriteTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8, pulltimestamp: *mut u64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISupportLastWriteTime_Impl::GetChangeUnitChangeTime(this, core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pbchangeunitid), core::mem::transmute_copy(&pulltimestamp)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetItemChangeTime: GetItemChangeTime::<Identity, OFFSET>,
            GetChangeUnitChangeTime: GetChangeUnitChangeTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISupportLastWriteTime as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISyncCallback, ISyncCallback_Vtbl, 0x0599797f_5ed9_485c_ae36_0c5d1bf2e7a5);
impl core::ops::Deref for ISyncCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISyncCallback, windows_core::IUnknown);
impl ISyncCallback {
    pub unsafe fn OnProgress(&self, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnProgress)(windows_core::Interface::as_raw(self), provider, syncstage, dwcompletedwork, dwtotalwork).ok()
    }
    pub unsafe fn OnChange<P0>(&self, psyncchange: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISyncChange>,
    {
        (windows_core::Interface::vtable(self).OnChange)(windows_core::Interface::as_raw(self), psyncchange.param().abi()).ok()
    }
    pub unsafe fn OnConflict<P0>(&self, pconflict: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IChangeConflict>,
    {
        (windows_core::Interface::vtable(self).OnConflict)(windows_core::Interface::as_raw(self), pconflict.param().abi()).ok()
    }
    pub unsafe fn OnFullEnumerationNeeded(&self, pfullenumerationaction: *mut SYNC_FULL_ENUMERATION_ACTION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnFullEnumerationNeeded)(windows_core::Interface::as_raw(self), pfullenumerationaction).ok()
    }
    pub unsafe fn OnRecoverableError<P0>(&self, precoverableerror: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRecoverableError>,
    {
        (windows_core::Interface::vtable(self).OnRecoverableError)(windows_core::Interface::as_raw(self), precoverableerror.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ISyncCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnProgress: unsafe extern "system" fn(*mut core::ffi::c_void, SYNC_PROVIDER_ROLE, SYNC_PROGRESS_STAGE, u32, u32) -> windows_core::HRESULT,
    pub OnChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnConflict: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnFullEnumerationNeeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SYNC_FULL_ENUMERATION_ACTION) -> windows_core::HRESULT,
    pub OnRecoverableError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISyncCallback_Impl: Sized + windows_core::IUnknownImpl {
    fn OnProgress(&self, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> windows_core::Result<()>;
    fn OnChange(&self, psyncchange: Option<&ISyncChange>) -> windows_core::Result<()>;
    fn OnConflict(&self, pconflict: Option<&IChangeConflict>) -> windows_core::Result<()>;
    fn OnFullEnumerationNeeded(&self, pfullenumerationaction: *mut SYNC_FULL_ENUMERATION_ACTION) -> windows_core::Result<()>;
    fn OnRecoverableError(&self, precoverableerror: Option<&IRecoverableError>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncCallback {}
impl ISyncCallback_Vtbl {
    pub const fn new<Identity: ISyncCallback_Impl, const OFFSET: isize>() -> ISyncCallback_Vtbl {
        unsafe extern "system" fn OnProgress<Identity: ISyncCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncCallback_Impl::OnProgress(this, core::mem::transmute_copy(&provider), core::mem::transmute_copy(&syncstage), core::mem::transmute_copy(&dwcompletedwork), core::mem::transmute_copy(&dwtotalwork)).into()
        }
        unsafe extern "system" fn OnChange<Identity: ISyncCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psyncchange: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncCallback_Impl::OnChange(this, windows_core::from_raw_borrowed(&psyncchange)).into()
        }
        unsafe extern "system" fn OnConflict<Identity: ISyncCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconflict: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncCallback_Impl::OnConflict(this, windows_core::from_raw_borrowed(&pconflict)).into()
        }
        unsafe extern "system" fn OnFullEnumerationNeeded<Identity: ISyncCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfullenumerationaction: *mut SYNC_FULL_ENUMERATION_ACTION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncCallback_Impl::OnFullEnumerationNeeded(this, core::mem::transmute_copy(&pfullenumerationaction)).into()
        }
        unsafe extern "system" fn OnRecoverableError<Identity: ISyncCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, precoverableerror: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncCallback_Impl::OnRecoverableError(this, windows_core::from_raw_borrowed(&precoverableerror)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnProgress: OnProgress::<Identity, OFFSET>,
            OnChange: OnChange::<Identity, OFFSET>,
            OnConflict: OnConflict::<Identity, OFFSET>,
            OnFullEnumerationNeeded: OnFullEnumerationNeeded::<Identity, OFFSET>,
            OnRecoverableError: OnRecoverableError::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncCallback as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISyncCallback2, ISyncCallback2_Vtbl, 0x47ce84af_7442_4ead_8630_12015e030ad7);
impl core::ops::Deref for ISyncCallback2 {
    type Target = ISyncCallback;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISyncCallback2, windows_core::IUnknown, ISyncCallback);
impl ISyncCallback2 {
    pub unsafe fn OnChangeApplied(&self, dwchangesapplied: u32, dwchangesfailed: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnChangeApplied)(windows_core::Interface::as_raw(self), dwchangesapplied, dwchangesfailed).ok()
    }
    pub unsafe fn OnChangeFailed(&self, dwchangesapplied: u32, dwchangesfailed: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnChangeFailed)(windows_core::Interface::as_raw(self), dwchangesapplied, dwchangesfailed).ok()
    }
}
#[repr(C)]
pub struct ISyncCallback2_Vtbl {
    pub base__: ISyncCallback_Vtbl,
    pub OnChangeApplied: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub OnChangeFailed: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
pub trait ISyncCallback2_Impl: Sized + ISyncCallback_Impl {
    fn OnChangeApplied(&self, dwchangesapplied: u32, dwchangesfailed: u32) -> windows_core::Result<()>;
    fn OnChangeFailed(&self, dwchangesapplied: u32, dwchangesfailed: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncCallback2 {}
impl ISyncCallback2_Vtbl {
    pub const fn new<Identity: ISyncCallback2_Impl, const OFFSET: isize>() -> ISyncCallback2_Vtbl {
        unsafe extern "system" fn OnChangeApplied<Identity: ISyncCallback2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwchangesapplied: u32, dwchangesfailed: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncCallback2_Impl::OnChangeApplied(this, core::mem::transmute_copy(&dwchangesapplied), core::mem::transmute_copy(&dwchangesfailed)).into()
        }
        unsafe extern "system" fn OnChangeFailed<Identity: ISyncCallback2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwchangesapplied: u32, dwchangesfailed: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncCallback2_Impl::OnChangeFailed(this, core::mem::transmute_copy(&dwchangesapplied), core::mem::transmute_copy(&dwchangesfailed)).into()
        }
        Self {
            base__: ISyncCallback_Vtbl::new::<Identity, OFFSET>(),
            OnChangeApplied: OnChangeApplied::<Identity, OFFSET>,
            OnChangeFailed: OnChangeFailed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncCallback2 as windows_core::Interface>::IID || iid == &<ISyncCallback as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISyncChange, ISyncChange_Vtbl, 0xa1952beb_0f6b_4711_b136_01da85b968a6);
impl core::ops::Deref for ISyncChange {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISyncChange, windows_core::IUnknown);
impl ISyncChange {
    pub unsafe fn GetOwnerReplicaId(&self, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetOwnerReplicaId)(windows_core::Interface::as_raw(self), pbreplicaid, pcbidsize).ok()
    }
    pub unsafe fn GetRootItemId(&self, pbrootitemid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRootItemId)(windows_core::Interface::as_raw(self), pbrootitemid, pcbidsize).ok()
    }
    pub unsafe fn GetChangeVersion(&self, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetChangeVersion)(windows_core::Interface::as_raw(self), pbcurrentreplicaid, pversion).ok()
    }
    pub unsafe fn GetCreationVersion(&self, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCreationVersion)(windows_core::Interface::as_raw(self), pbcurrentreplicaid, pversion).ok()
    }
    pub unsafe fn GetFlags(&self, pdwflags: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFlags)(windows_core::Interface::as_raw(self), pdwflags).ok()
    }
    pub unsafe fn GetWorkEstimate(&self, pdwwork: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetWorkEstimate)(windows_core::Interface::as_raw(self), pdwwork).ok()
    }
    pub unsafe fn GetChangeUnits(&self) -> windows_core::Result<IEnumSyncChangeUnits> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetChangeUnits)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetMadeWithKnowledge(&self) -> windows_core::Result<ISyncKnowledge> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMadeWithKnowledge)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetLearnedKnowledge(&self) -> windows_core::Result<ISyncKnowledge> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLearnedKnowledge)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetWorkEstimate(&self, dwwork: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetWorkEstimate)(windows_core::Interface::as_raw(self), dwwork).ok()
    }
}
#[repr(C)]
pub struct ISyncChange_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetOwnerReplicaId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetRootItemId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetChangeVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *mut SYNC_VERSION) -> windows_core::HRESULT,
    pub GetCreationVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *mut SYNC_VERSION) -> windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetWorkEstimate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetChangeUnits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMadeWithKnowledge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLearnedKnowledge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWorkEstimate: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait ISyncChange_Impl: Sized + windows_core::IUnknownImpl {
    fn GetOwnerReplicaId(&self, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()>;
    fn GetRootItemId(&self, pbrootitemid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()>;
    fn GetChangeVersion(&self, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> windows_core::Result<()>;
    fn GetCreationVersion(&self, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> windows_core::Result<()>;
    fn GetFlags(&self, pdwflags: *mut u32) -> windows_core::Result<()>;
    fn GetWorkEstimate(&self, pdwwork: *mut u32) -> windows_core::Result<()>;
    fn GetChangeUnits(&self) -> windows_core::Result<IEnumSyncChangeUnits>;
    fn GetMadeWithKnowledge(&self) -> windows_core::Result<ISyncKnowledge>;
    fn GetLearnedKnowledge(&self) -> windows_core::Result<ISyncKnowledge>;
    fn SetWorkEstimate(&self, dwwork: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncChange {}
impl ISyncChange_Vtbl {
    pub const fn new<Identity: ISyncChange_Impl, const OFFSET: isize>() -> ISyncChange_Vtbl {
        unsafe extern "system" fn GetOwnerReplicaId<Identity: ISyncChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChange_Impl::GetOwnerReplicaId(this, core::mem::transmute_copy(&pbreplicaid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetRootItemId<Identity: ISyncChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbrootitemid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChange_Impl::GetRootItemId(this, core::mem::transmute_copy(&pbrootitemid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetChangeVersion<Identity: ISyncChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChange_Impl::GetChangeVersion(this, core::mem::transmute_copy(&pbcurrentreplicaid), core::mem::transmute_copy(&pversion)).into()
        }
        unsafe extern "system" fn GetCreationVersion<Identity: ISyncChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChange_Impl::GetCreationVersion(this, core::mem::transmute_copy(&pbcurrentreplicaid), core::mem::transmute_copy(&pversion)).into()
        }
        unsafe extern "system" fn GetFlags<Identity: ISyncChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChange_Impl::GetFlags(this, core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn GetWorkEstimate<Identity: ISyncChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwwork: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChange_Impl::GetWorkEstimate(this, core::mem::transmute_copy(&pdwwork)).into()
        }
        unsafe extern "system" fn GetChangeUnits<Identity: ISyncChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChange_Impl::GetChangeUnits(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMadeWithKnowledge<Identity: ISyncChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmadewithknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChange_Impl::GetMadeWithKnowledge(this) {
                Ok(ok__) => {
                    ppmadewithknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedKnowledge<Identity: ISyncChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplearnedknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChange_Impl::GetLearnedKnowledge(this) {
                Ok(ok__) => {
                    pplearnedknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWorkEstimate<Identity: ISyncChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwwork: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChange_Impl::SetWorkEstimate(this, core::mem::transmute_copy(&dwwork)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOwnerReplicaId: GetOwnerReplicaId::<Identity, OFFSET>,
            GetRootItemId: GetRootItemId::<Identity, OFFSET>,
            GetChangeVersion: GetChangeVersion::<Identity, OFFSET>,
            GetCreationVersion: GetCreationVersion::<Identity, OFFSET>,
            GetFlags: GetFlags::<Identity, OFFSET>,
            GetWorkEstimate: GetWorkEstimate::<Identity, OFFSET>,
            GetChangeUnits: GetChangeUnits::<Identity, OFFSET>,
            GetMadeWithKnowledge: GetMadeWithKnowledge::<Identity, OFFSET>,
            GetLearnedKnowledge: GetLearnedKnowledge::<Identity, OFFSET>,
            SetWorkEstimate: SetWorkEstimate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncChange as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISyncChangeBatch, ISyncChangeBatch_Vtbl, 0x70c64dee_380f_4c2e_8f70_31c55bd5f9b3);
impl core::ops::Deref for ISyncChangeBatch {
    type Target = ISyncChangeBatchBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISyncChangeBatch, windows_core::IUnknown, ISyncChangeBatchBase);
impl ISyncChangeBatch {
    pub unsafe fn BeginUnorderedGroup(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BeginUnorderedGroup)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EndUnorderedGroup<P0, P1>(&self, pmadewithknowledge: P0, fallchangesforknowledge: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISyncKnowledge>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).EndUnorderedGroup)(windows_core::Interface::as_raw(self), pmadewithknowledge.param().abi(), fallchangesforknowledge.param().abi()).ok()
    }
    pub unsafe fn AddLoggedConflict<P0>(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, pconflictknowledge: P0) -> windows_core::Result<ISyncChangeBuilder>
    where
        P0: windows_core::Param<ISyncKnowledge>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddLoggedConflict)(windows_core::Interface::as_raw(self), pbownerreplicaid, pbitemid, pchangeversion, pcreationversion, dwflags, dwworkforchange, pconflictknowledge.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISyncChangeBatch_Vtbl {
    pub base__: ISyncChangeBatchBase_Vtbl,
    pub BeginUnorderedGroup: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndUnorderedGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub AddLoggedConflict: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *const u8, *const SYNC_VERSION, *const SYNC_VERSION, u32, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISyncChangeBatch_Impl: Sized + ISyncChangeBatchBase_Impl {
    fn BeginUnorderedGroup(&self) -> windows_core::Result<()>;
    fn EndUnorderedGroup(&self, pmadewithknowledge: Option<&ISyncKnowledge>, fallchangesforknowledge: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn AddLoggedConflict(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, pconflictknowledge: Option<&ISyncKnowledge>) -> windows_core::Result<ISyncChangeBuilder>;
}
impl windows_core::RuntimeName for ISyncChangeBatch {}
impl ISyncChangeBatch_Vtbl {
    pub const fn new<Identity: ISyncChangeBatch_Impl, const OFFSET: isize>() -> ISyncChangeBatch_Vtbl {
        unsafe extern "system" fn BeginUnorderedGroup<Identity: ISyncChangeBatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeBatch_Impl::BeginUnorderedGroup(this).into()
        }
        unsafe extern "system" fn EndUnorderedGroup<Identity: ISyncChangeBatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmadewithknowledge: *mut core::ffi::c_void, fallchangesforknowledge: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeBatch_Impl::EndUnorderedGroup(this, windows_core::from_raw_borrowed(&pmadewithknowledge), core::mem::transmute_copy(&fallchangesforknowledge)).into()
        }
        unsafe extern "system" fn AddLoggedConflict<Identity: ISyncChangeBatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, pconflictknowledge: *mut core::ffi::c_void, ppchangebuilder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeBatch_Impl::AddLoggedConflict(this, core::mem::transmute_copy(&pbownerreplicaid), core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pchangeversion), core::mem::transmute_copy(&pcreationversion), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwworkforchange), windows_core::from_raw_borrowed(&pconflictknowledge)) {
                Ok(ok__) => {
                    ppchangebuilder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ISyncChangeBatchBase_Vtbl::new::<Identity, OFFSET>(),
            BeginUnorderedGroup: BeginUnorderedGroup::<Identity, OFFSET>,
            EndUnorderedGroup: EndUnorderedGroup::<Identity, OFFSET>,
            AddLoggedConflict: AddLoggedConflict::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncChangeBatch as windows_core::Interface>::IID || iid == &<ISyncChangeBatchBase as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISyncChangeBatch2, ISyncChangeBatch2_Vtbl, 0x225f4a33_f5ee_4cc7_b039_67a262b4b2ac);
impl core::ops::Deref for ISyncChangeBatch2 {
    type Target = ISyncChangeBatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISyncChangeBatch2, windows_core::IUnknown, ISyncChangeBatchBase, ISyncChangeBatch);
impl ISyncChangeBatch2 {
    pub unsafe fn AddMergeTombstoneMetadataToGroup(&self, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32) -> windows_core::Result<ISyncChangeBuilder> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddMergeTombstoneMetadataToGroup)(windows_core::Interface::as_raw(self), pbownerreplicaid, pbwinneritemid, pbitemid, pchangeversion, pcreationversion, dwworkforchange, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddMergeTombstoneLoggedConflict<P0>(&self, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, pconflictknowledge: P0) -> windows_core::Result<ISyncChangeBuilder>
    where
        P0: windows_core::Param<ISyncKnowledge>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddMergeTombstoneLoggedConflict)(windows_core::Interface::as_raw(self), pbownerreplicaid, pbwinneritemid, pbitemid, pchangeversion, pcreationversion, dwworkforchange, pconflictknowledge.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISyncChangeBatch2_Vtbl {
    pub base__: ISyncChangeBatch_Vtbl,
    pub AddMergeTombstoneMetadataToGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *const u8, *const u8, *const SYNC_VERSION, *const SYNC_VERSION, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddMergeTombstoneLoggedConflict: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *const u8, *const u8, *const SYNC_VERSION, *const SYNC_VERSION, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISyncChangeBatch2_Impl: Sized + ISyncChangeBatch_Impl {
    fn AddMergeTombstoneMetadataToGroup(&self, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32) -> windows_core::Result<ISyncChangeBuilder>;
    fn AddMergeTombstoneLoggedConflict(&self, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, pconflictknowledge: Option<&ISyncKnowledge>) -> windows_core::Result<ISyncChangeBuilder>;
}
impl windows_core::RuntimeName for ISyncChangeBatch2 {}
impl ISyncChangeBatch2_Vtbl {
    pub const fn new<Identity: ISyncChangeBatch2_Impl, const OFFSET: isize>() -> ISyncChangeBatch2_Vtbl {
        unsafe extern "system" fn AddMergeTombstoneMetadataToGroup<Identity: ISyncChangeBatch2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, ppchangebuilder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeBatch2_Impl::AddMergeTombstoneMetadataToGroup(this, core::mem::transmute_copy(&pbownerreplicaid), core::mem::transmute_copy(&pbwinneritemid), core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pchangeversion), core::mem::transmute_copy(&pcreationversion), core::mem::transmute_copy(&dwworkforchange)) {
                Ok(ok__) => {
                    ppchangebuilder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddMergeTombstoneLoggedConflict<Identity: ISyncChangeBatch2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, pconflictknowledge: *mut core::ffi::c_void, ppchangebuilder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeBatch2_Impl::AddMergeTombstoneLoggedConflict(this, core::mem::transmute_copy(&pbownerreplicaid), core::mem::transmute_copy(&pbwinneritemid), core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pchangeversion), core::mem::transmute_copy(&pcreationversion), core::mem::transmute_copy(&dwworkforchange), windows_core::from_raw_borrowed(&pconflictknowledge)) {
                Ok(ok__) => {
                    ppchangebuilder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ISyncChangeBatch_Vtbl::new::<Identity, OFFSET>(),
            AddMergeTombstoneMetadataToGroup: AddMergeTombstoneMetadataToGroup::<Identity, OFFSET>,
            AddMergeTombstoneLoggedConflict: AddMergeTombstoneLoggedConflict::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncChangeBatch2 as windows_core::Interface>::IID || iid == &<ISyncChangeBatchBase as windows_core::Interface>::IID || iid == &<ISyncChangeBatch as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISyncChangeBatchAdvanced, ISyncChangeBatchAdvanced_Vtbl, 0x0f1a4995_cbc8_421d_b550_5d0bebf3e9a5);
impl core::ops::Deref for ISyncChangeBatchAdvanced {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISyncChangeBatchAdvanced, windows_core::IUnknown);
impl ISyncChangeBatchAdvanced {
    pub unsafe fn GetFilterInfo(&self) -> windows_core::Result<ISyncFilterInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFilterInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ConvertFullEnumerationChangeBatchToRegularChangeBatch(&self) -> windows_core::Result<ISyncChangeBatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ConvertFullEnumerationChangeBatchToRegularChangeBatch)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetUpperBoundItemId(&self, pbitemid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetUpperBoundItemId)(windows_core::Interface::as_raw(self), pbitemid, pcbidsize).ok()
    }
    pub unsafe fn GetBatchLevelKnowledgeShouldBeApplied(&self, pfbatchknowledgeshouldbeapplied: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBatchLevelKnowledgeShouldBeApplied)(windows_core::Interface::as_raw(self), pfbatchknowledgeshouldbeapplied).ok()
    }
}
#[repr(C)]
pub struct ISyncChangeBatchAdvanced_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFilterInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConvertFullEnumerationChangeBatchToRegularChangeBatch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetUpperBoundItemId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetBatchLevelKnowledgeShouldBeApplied: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
pub trait ISyncChangeBatchAdvanced_Impl: Sized + windows_core::IUnknownImpl {
    fn GetFilterInfo(&self) -> windows_core::Result<ISyncFilterInfo>;
    fn ConvertFullEnumerationChangeBatchToRegularChangeBatch(&self) -> windows_core::Result<ISyncChangeBatch>;
    fn GetUpperBoundItemId(&self, pbitemid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()>;
    fn GetBatchLevelKnowledgeShouldBeApplied(&self, pfbatchknowledgeshouldbeapplied: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncChangeBatchAdvanced {}
impl ISyncChangeBatchAdvanced_Vtbl {
    pub const fn new<Identity: ISyncChangeBatchAdvanced_Impl, const OFFSET: isize>() -> ISyncChangeBatchAdvanced_Vtbl {
        unsafe extern "system" fn GetFilterInfo<Identity: ISyncChangeBatchAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfilterinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeBatchAdvanced_Impl::GetFilterInfo(this) {
                Ok(ok__) => {
                    ppfilterinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertFullEnumerationChangeBatchToRegularChangeBatch<Identity: ISyncChangeBatchAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppchangebatch: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeBatchAdvanced_Impl::ConvertFullEnumerationChangeBatchToRegularChangeBatch(this) {
                Ok(ok__) => {
                    ppchangebatch.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpperBoundItemId<Identity: ISyncChangeBatchAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitemid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeBatchAdvanced_Impl::GetUpperBoundItemId(this, core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetBatchLevelKnowledgeShouldBeApplied<Identity: ISyncChangeBatchAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfbatchknowledgeshouldbeapplied: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeBatchAdvanced_Impl::GetBatchLevelKnowledgeShouldBeApplied(this, core::mem::transmute_copy(&pfbatchknowledgeshouldbeapplied)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFilterInfo: GetFilterInfo::<Identity, OFFSET>,
            ConvertFullEnumerationChangeBatchToRegularChangeBatch: ConvertFullEnumerationChangeBatchToRegularChangeBatch::<Identity, OFFSET>,
            GetUpperBoundItemId: GetUpperBoundItemId::<Identity, OFFSET>,
            GetBatchLevelKnowledgeShouldBeApplied: GetBatchLevelKnowledgeShouldBeApplied::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncChangeBatchAdvanced as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISyncChangeBatchBase, ISyncChangeBatchBase_Vtbl, 0x52f6e694_6a71_4494_a184_a8311bf5d227);
impl core::ops::Deref for ISyncChangeBatchBase {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISyncChangeBatchBase, windows_core::IUnknown);
impl ISyncChangeBatchBase {
    pub unsafe fn GetChangeEnumerator(&self) -> windows_core::Result<IEnumSyncChanges> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetChangeEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetIsLastBatch(&self, pflastbatch: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetIsLastBatch)(windows_core::Interface::as_raw(self), pflastbatch).ok()
    }
    pub unsafe fn GetWorkEstimateForBatch(&self, pdwworkforbatch: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetWorkEstimateForBatch)(windows_core::Interface::as_raw(self), pdwworkforbatch).ok()
    }
    pub unsafe fn GetRemainingWorkEstimateForSession(&self, pdwremainingworkforsession: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRemainingWorkEstimateForSession)(windows_core::Interface::as_raw(self), pdwremainingworkforsession).ok()
    }
    pub unsafe fn BeginOrderedGroup(&self, pblowerbound: *const u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BeginOrderedGroup)(windows_core::Interface::as_raw(self), pblowerbound).ok()
    }
    pub unsafe fn EndOrderedGroup<P0>(&self, pbupperbound: *const u8, pmadewithknowledge: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISyncKnowledge>,
    {
        (windows_core::Interface::vtable(self).EndOrderedGroup)(windows_core::Interface::as_raw(self), pbupperbound, pmadewithknowledge.param().abi()).ok()
    }
    pub unsafe fn AddItemMetadataToGroup(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32) -> windows_core::Result<ISyncChangeBuilder> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddItemMetadataToGroup)(windows_core::Interface::as_raw(self), pbownerreplicaid, pbitemid, pchangeversion, pcreationversion, dwflags, dwworkforchange, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetLearnedKnowledge(&self) -> windows_core::Result<ISyncKnowledge> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLearnedKnowledge)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPrerequisiteKnowledge(&self) -> windows_core::Result<ISyncKnowledge> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPrerequisiteKnowledge)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSourceForgottenKnowledge(&self) -> windows_core::Result<IForgottenKnowledge> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSourceForgottenKnowledge)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLastBatch(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLastBatch)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetWorkEstimateForBatch(&self, dwworkforbatch: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetWorkEstimateForBatch)(windows_core::Interface::as_raw(self), dwworkforbatch).ok()
    }
    pub unsafe fn SetRemainingWorkEstimateForSession(&self, dwremainingworkforsession: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRemainingWorkEstimateForSession)(windows_core::Interface::as_raw(self), dwremainingworkforsession).ok()
    }
    pub unsafe fn Serialize(&self, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Serialize)(windows_core::Interface::as_raw(self), pbchangebatch, pcbchangebatch).ok()
    }
}
#[repr(C)]
pub struct ISyncChangeBatchBase_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetChangeEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIsLastBatch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetWorkEstimateForBatch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetRemainingWorkEstimateForSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub BeginOrderedGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8) -> windows_core::HRESULT,
    pub EndOrderedGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddItemMetadataToGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *const u8, *const SYNC_VERSION, *const SYNC_VERSION, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLearnedKnowledge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPrerequisiteKnowledge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSourceForgottenKnowledge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLastBatch: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWorkEstimateForBatch: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetRemainingWorkEstimateForSession: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Serialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
}
pub trait ISyncChangeBatchBase_Impl: Sized + windows_core::IUnknownImpl {
    fn GetChangeEnumerator(&self) -> windows_core::Result<IEnumSyncChanges>;
    fn GetIsLastBatch(&self, pflastbatch: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetWorkEstimateForBatch(&self, pdwworkforbatch: *mut u32) -> windows_core::Result<()>;
    fn GetRemainingWorkEstimateForSession(&self, pdwremainingworkforsession: *mut u32) -> windows_core::Result<()>;
    fn BeginOrderedGroup(&self, pblowerbound: *const u8) -> windows_core::Result<()>;
    fn EndOrderedGroup(&self, pbupperbound: *const u8, pmadewithknowledge: Option<&ISyncKnowledge>) -> windows_core::Result<()>;
    fn AddItemMetadataToGroup(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32) -> windows_core::Result<ISyncChangeBuilder>;
    fn GetLearnedKnowledge(&self) -> windows_core::Result<ISyncKnowledge>;
    fn GetPrerequisiteKnowledge(&self) -> windows_core::Result<ISyncKnowledge>;
    fn GetSourceForgottenKnowledge(&self) -> windows_core::Result<IForgottenKnowledge>;
    fn SetLastBatch(&self) -> windows_core::Result<()>;
    fn SetWorkEstimateForBatch(&self, dwworkforbatch: u32) -> windows_core::Result<()>;
    fn SetRemainingWorkEstimateForSession(&self, dwremainingworkforsession: u32) -> windows_core::Result<()>;
    fn Serialize(&self, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncChangeBatchBase {}
impl ISyncChangeBatchBase_Vtbl {
    pub const fn new<Identity: ISyncChangeBatchBase_Impl, const OFFSET: isize>() -> ISyncChangeBatchBase_Vtbl {
        unsafe extern "system" fn GetChangeEnumerator<Identity: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeBatchBase_Impl::GetChangeEnumerator(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsLastBatch<Identity: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflastbatch: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeBatchBase_Impl::GetIsLastBatch(this, core::mem::transmute_copy(&pflastbatch)).into()
        }
        unsafe extern "system" fn GetWorkEstimateForBatch<Identity: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwworkforbatch: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeBatchBase_Impl::GetWorkEstimateForBatch(this, core::mem::transmute_copy(&pdwworkforbatch)).into()
        }
        unsafe extern "system" fn GetRemainingWorkEstimateForSession<Identity: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwremainingworkforsession: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeBatchBase_Impl::GetRemainingWorkEstimateForSession(this, core::mem::transmute_copy(&pdwremainingworkforsession)).into()
        }
        unsafe extern "system" fn BeginOrderedGroup<Identity: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblowerbound: *const u8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeBatchBase_Impl::BeginOrderedGroup(this, core::mem::transmute_copy(&pblowerbound)).into()
        }
        unsafe extern "system" fn EndOrderedGroup<Identity: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbupperbound: *const u8, pmadewithknowledge: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeBatchBase_Impl::EndOrderedGroup(this, core::mem::transmute_copy(&pbupperbound), windows_core::from_raw_borrowed(&pmadewithknowledge)).into()
        }
        unsafe extern "system" fn AddItemMetadataToGroup<Identity: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, ppchangebuilder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeBatchBase_Impl::AddItemMetadataToGroup(this, core::mem::transmute_copy(&pbownerreplicaid), core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pchangeversion), core::mem::transmute_copy(&pcreationversion), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwworkforchange)) {
                Ok(ok__) => {
                    ppchangebuilder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedKnowledge<Identity: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplearnedknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeBatchBase_Impl::GetLearnedKnowledge(this) {
                Ok(ok__) => {
                    pplearnedknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrerequisiteKnowledge<Identity: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprerequisteknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeBatchBase_Impl::GetPrerequisiteKnowledge(this) {
                Ok(ok__) => {
                    ppprerequisteknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceForgottenKnowledge<Identity: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsourceforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeBatchBase_Impl::GetSourceForgottenKnowledge(this) {
                Ok(ok__) => {
                    ppsourceforgottenknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastBatch<Identity: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeBatchBase_Impl::SetLastBatch(this).into()
        }
        unsafe extern "system" fn SetWorkEstimateForBatch<Identity: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwworkforbatch: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeBatchBase_Impl::SetWorkEstimateForBatch(this, core::mem::transmute_copy(&dwworkforbatch)).into()
        }
        unsafe extern "system" fn SetRemainingWorkEstimateForSession<Identity: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwremainingworkforsession: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeBatchBase_Impl::SetRemainingWorkEstimateForSession(this, core::mem::transmute_copy(&dwremainingworkforsession)).into()
        }
        unsafe extern "system" fn Serialize<Identity: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeBatchBase_Impl::Serialize(this, core::mem::transmute_copy(&pbchangebatch), core::mem::transmute_copy(&pcbchangebatch)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetChangeEnumerator: GetChangeEnumerator::<Identity, OFFSET>,
            GetIsLastBatch: GetIsLastBatch::<Identity, OFFSET>,
            GetWorkEstimateForBatch: GetWorkEstimateForBatch::<Identity, OFFSET>,
            GetRemainingWorkEstimateForSession: GetRemainingWorkEstimateForSession::<Identity, OFFSET>,
            BeginOrderedGroup: BeginOrderedGroup::<Identity, OFFSET>,
            EndOrderedGroup: EndOrderedGroup::<Identity, OFFSET>,
            AddItemMetadataToGroup: AddItemMetadataToGroup::<Identity, OFFSET>,
            GetLearnedKnowledge: GetLearnedKnowledge::<Identity, OFFSET>,
            GetPrerequisiteKnowledge: GetPrerequisiteKnowledge::<Identity, OFFSET>,
            GetSourceForgottenKnowledge: GetSourceForgottenKnowledge::<Identity, OFFSET>,
            SetLastBatch: SetLastBatch::<Identity, OFFSET>,
            SetWorkEstimateForBatch: SetWorkEstimateForBatch::<Identity, OFFSET>,
            SetRemainingWorkEstimateForSession: SetRemainingWorkEstimateForSession::<Identity, OFFSET>,
            Serialize: Serialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncChangeBatchBase as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISyncChangeBatchBase2, ISyncChangeBatchBase2_Vtbl, 0x6fdb596a_d755_4584_bd0c_c0c23a548fbf);
impl core::ops::Deref for ISyncChangeBatchBase2 {
    type Target = ISyncChangeBatchBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISyncChangeBatchBase2, windows_core::IUnknown, ISyncChangeBatchBase);
impl ISyncChangeBatchBase2 {
    pub unsafe fn SerializeWithOptions(&self, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SerializeWithOptions)(windows_core::Interface::as_raw(self), targetformatversion, dwflags, pbbuffer, pdwserializedsize).ok()
    }
}
#[repr(C)]
pub struct ISyncChangeBatchBase2_Vtbl {
    pub base__: ISyncChangeBatchBase_Vtbl,
    pub SerializeWithOptions: unsafe extern "system" fn(*mut core::ffi::c_void, SYNC_SERIALIZATION_VERSION, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
}
pub trait ISyncChangeBatchBase2_Impl: Sized + ISyncChangeBatchBase_Impl {
    fn SerializeWithOptions(&self, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncChangeBatchBase2 {}
impl ISyncChangeBatchBase2_Vtbl {
    pub const fn new<Identity: ISyncChangeBatchBase2_Impl, const OFFSET: isize>() -> ISyncChangeBatchBase2_Vtbl {
        unsafe extern "system" fn SerializeWithOptions<Identity: ISyncChangeBatchBase2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeBatchBase2_Impl::SerializeWithOptions(this, core::mem::transmute_copy(&targetformatversion), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pbbuffer), core::mem::transmute_copy(&pdwserializedsize)).into()
        }
        Self { base__: ISyncChangeBatchBase_Vtbl::new::<Identity, OFFSET>(), SerializeWithOptions: SerializeWithOptions::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncChangeBatchBase2 as windows_core::Interface>::IID || iid == &<ISyncChangeBatchBase as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISyncChangeBatchWithFilterKeyMap, ISyncChangeBatchWithFilterKeyMap_Vtbl, 0xde247002_566d_459a_a6ed_a5aab3459fb7);
impl core::ops::Deref for ISyncChangeBatchWithFilterKeyMap {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISyncChangeBatchWithFilterKeyMap, windows_core::IUnknown);
impl ISyncChangeBatchWithFilterKeyMap {
    pub unsafe fn GetFilterKeyMap(&self) -> windows_core::Result<IFilterKeyMap> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFilterKeyMap)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFilterKeyMap<P0>(&self, pifilterkeymap: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFilterKeyMap>,
    {
        (windows_core::Interface::vtable(self).SetFilterKeyMap)(windows_core::Interface::as_raw(self), pifilterkeymap.param().abi()).ok()
    }
    pub unsafe fn SetFilterForgottenKnowledge<P0>(&self, dwfilterkey: u32, pfilterforgottenknowledge: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISyncKnowledge>,
    {
        (windows_core::Interface::vtable(self).SetFilterForgottenKnowledge)(windows_core::Interface::as_raw(self), dwfilterkey, pfilterforgottenknowledge.param().abi()).ok()
    }
    pub unsafe fn GetFilteredReplicaLearnedKnowledge<P0, P1>(&self, pdestinationknowledge: P0, pnewmoveins: P1) -> windows_core::Result<ISyncKnowledge>
    where
        P0: windows_core::Param<ISyncKnowledge>,
        P1: windows_core::Param<IEnumItemIds>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFilteredReplicaLearnedKnowledge)(windows_core::Interface::as_raw(self), pdestinationknowledge.param().abi(), pnewmoveins.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetLearnedFilterForgottenKnowledge<P0, P1>(&self, pdestinationknowledge: P0, pnewmoveins: P1, dwfilterkey: u32) -> windows_core::Result<ISyncKnowledge>
    where
        P0: windows_core::Param<ISyncKnowledge>,
        P1: windows_core::Param<IEnumItemIds>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLearnedFilterForgottenKnowledge)(windows_core::Interface::as_raw(self), pdestinationknowledge.param().abi(), pnewmoveins.param().abi(), dwfilterkey, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFilteredReplicaLearnedForgottenKnowledge<P0, P1>(&self, pdestinationknowledge: P0, pnewmoveins: P1) -> windows_core::Result<ISyncKnowledge>
    where
        P0: windows_core::Param<ISyncKnowledge>,
        P1: windows_core::Param<IEnumItemIds>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFilteredReplicaLearnedForgottenKnowledge)(windows_core::Interface::as_raw(self), pdestinationknowledge.param().abi(), pnewmoveins.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete<P0, P1>(&self, pdestinationknowledge: P0, pnewmoveins: P1) -> windows_core::Result<ISyncKnowledge>
    where
        P0: windows_core::Param<ISyncKnowledge>,
        P1: windows_core::Param<IEnumItemIds>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete)(windows_core::Interface::as_raw(self), pdestinationknowledge.param().abi(), pnewmoveins.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete<P0, P1>(&self, pdestinationknowledge: P0, pnewmoveins: P1, dwfilterkey: u32) -> windows_core::Result<ISyncKnowledge>
    where
        P0: windows_core::Param<ISyncKnowledge>,
        P1: windows_core::Param<IEnumItemIds>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete)(windows_core::Interface::as_raw(self), pdestinationknowledge.param().abi(), pnewmoveins.param().abi(), dwfilterkey, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISyncChangeBatchWithFilterKeyMap_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFilterKeyMap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFilterKeyMap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFilterForgottenKnowledge: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFilteredReplicaLearnedKnowledge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLearnedFilterForgottenKnowledge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFilteredReplicaLearnedForgottenKnowledge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISyncChangeBatchWithFilterKeyMap_Impl: Sized + windows_core::IUnknownImpl {
    fn GetFilterKeyMap(&self) -> windows_core::Result<IFilterKeyMap>;
    fn SetFilterKeyMap(&self, pifilterkeymap: Option<&IFilterKeyMap>) -> windows_core::Result<()>;
    fn SetFilterForgottenKnowledge(&self, dwfilterkey: u32, pfilterforgottenknowledge: Option<&ISyncKnowledge>) -> windows_core::Result<()>;
    fn GetFilteredReplicaLearnedKnowledge(&self, pdestinationknowledge: Option<&ISyncKnowledge>, pnewmoveins: Option<&IEnumItemIds>) -> windows_core::Result<ISyncKnowledge>;
    fn GetLearnedFilterForgottenKnowledge(&self, pdestinationknowledge: Option<&ISyncKnowledge>, pnewmoveins: Option<&IEnumItemIds>, dwfilterkey: u32) -> windows_core::Result<ISyncKnowledge>;
    fn GetFilteredReplicaLearnedForgottenKnowledge(&self, pdestinationknowledge: Option<&ISyncKnowledge>, pnewmoveins: Option<&IEnumItemIds>) -> windows_core::Result<ISyncKnowledge>;
    fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete(&self, pdestinationknowledge: Option<&ISyncKnowledge>, pnewmoveins: Option<&IEnumItemIds>) -> windows_core::Result<ISyncKnowledge>;
    fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete(&self, pdestinationknowledge: Option<&ISyncKnowledge>, pnewmoveins: Option<&IEnumItemIds>, dwfilterkey: u32) -> windows_core::Result<ISyncKnowledge>;
}
impl windows_core::RuntimeName for ISyncChangeBatchWithFilterKeyMap {}
impl ISyncChangeBatchWithFilterKeyMap_Vtbl {
    pub const fn new<Identity: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: isize>() -> ISyncChangeBatchWithFilterKeyMap_Vtbl {
        unsafe extern "system" fn GetFilterKeyMap<Identity: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppifilterkeymap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeBatchWithFilterKeyMap_Impl::GetFilterKeyMap(this) {
                Ok(ok__) => {
                    ppifilterkeymap.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFilterKeyMap<Identity: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pifilterkeymap: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeBatchWithFilterKeyMap_Impl::SetFilterKeyMap(this, windows_core::from_raw_borrowed(&pifilterkeymap)).into()
        }
        unsafe extern "system" fn SetFilterForgottenKnowledge<Identity: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfilterkey: u32, pfilterforgottenknowledge: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeBatchWithFilterKeyMap_Impl::SetFilterForgottenKnowledge(this, core::mem::transmute_copy(&dwfilterkey), windows_core::from_raw_borrowed(&pfilterforgottenknowledge)).into()
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedKnowledge<Identity: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationknowledge: *mut core::ffi::c_void, pnewmoveins: *mut core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeBatchWithFilterKeyMap_Impl::GetFilteredReplicaLearnedKnowledge(this, windows_core::from_raw_borrowed(&pdestinationknowledge), windows_core::from_raw_borrowed(&pnewmoveins)) {
                Ok(ok__) => {
                    pplearnedforgottenknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedFilterForgottenKnowledge<Identity: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationknowledge: *mut core::ffi::c_void, pnewmoveins: *mut core::ffi::c_void, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeBatchWithFilterKeyMap_Impl::GetLearnedFilterForgottenKnowledge(this, windows_core::from_raw_borrowed(&pdestinationknowledge), windows_core::from_raw_borrowed(&pnewmoveins), core::mem::transmute_copy(&dwfilterkey)) {
                Ok(ok__) => {
                    pplearnedfilterforgottenknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedForgottenKnowledge<Identity: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationknowledge: *mut core::ffi::c_void, pnewmoveins: *mut core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeBatchWithFilterKeyMap_Impl::GetFilteredReplicaLearnedForgottenKnowledge(this, windows_core::from_raw_borrowed(&pdestinationknowledge), windows_core::from_raw_borrowed(&pnewmoveins)) {
                Ok(ok__) => {
                    pplearnedforgottenknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete<Identity: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationknowledge: *mut core::ffi::c_void, pnewmoveins: *mut core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeBatchWithFilterKeyMap_Impl::GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete(this, windows_core::from_raw_borrowed(&pdestinationknowledge), windows_core::from_raw_borrowed(&pnewmoveins)) {
                Ok(ok__) => {
                    pplearnedforgottenknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete<Identity: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationknowledge: *mut core::ffi::c_void, pnewmoveins: *mut core::ffi::c_void, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeBatchWithFilterKeyMap_Impl::GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete(this, windows_core::from_raw_borrowed(&pdestinationknowledge), windows_core::from_raw_borrowed(&pnewmoveins), core::mem::transmute_copy(&dwfilterkey)) {
                Ok(ok__) => {
                    pplearnedfilterforgottenknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFilterKeyMap: GetFilterKeyMap::<Identity, OFFSET>,
            SetFilterKeyMap: SetFilterKeyMap::<Identity, OFFSET>,
            SetFilterForgottenKnowledge: SetFilterForgottenKnowledge::<Identity, OFFSET>,
            GetFilteredReplicaLearnedKnowledge: GetFilteredReplicaLearnedKnowledge::<Identity, OFFSET>,
            GetLearnedFilterForgottenKnowledge: GetLearnedFilterForgottenKnowledge::<Identity, OFFSET>,
            GetFilteredReplicaLearnedForgottenKnowledge: GetFilteredReplicaLearnedForgottenKnowledge::<Identity, OFFSET>,
            GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete: GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete::<Identity, OFFSET>,
            GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete: GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncChangeBatchWithFilterKeyMap as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISyncChangeBatchWithPrerequisite, ISyncChangeBatchWithPrerequisite_Vtbl, 0x097f13be_5b92_4048_b3f2_7b42a2515e07);
impl core::ops::Deref for ISyncChangeBatchWithPrerequisite {
    type Target = ISyncChangeBatchBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISyncChangeBatchWithPrerequisite, windows_core::IUnknown, ISyncChangeBatchBase);
impl ISyncChangeBatchWithPrerequisite {
    pub unsafe fn SetPrerequisiteKnowledge<P0>(&self, pprerequisiteknowledge: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISyncKnowledge>,
    {
        (windows_core::Interface::vtable(self).SetPrerequisiteKnowledge)(windows_core::Interface::as_raw(self), pprerequisiteknowledge.param().abi()).ok()
    }
    pub unsafe fn GetLearnedKnowledgeWithPrerequisite<P0>(&self, pdestinationknowledge: P0) -> windows_core::Result<ISyncKnowledge>
    where
        P0: windows_core::Param<ISyncKnowledge>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLearnedKnowledgeWithPrerequisite)(windows_core::Interface::as_raw(self), pdestinationknowledge.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetLearnedForgottenKnowledge(&self) -> windows_core::Result<IForgottenKnowledge> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLearnedForgottenKnowledge)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISyncChangeBatchWithPrerequisite_Vtbl {
    pub base__: ISyncChangeBatchBase_Vtbl,
    pub SetPrerequisiteKnowledge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLearnedKnowledgeWithPrerequisite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLearnedForgottenKnowledge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISyncChangeBatchWithPrerequisite_Impl: Sized + ISyncChangeBatchBase_Impl {
    fn SetPrerequisiteKnowledge(&self, pprerequisiteknowledge: Option<&ISyncKnowledge>) -> windows_core::Result<()>;
    fn GetLearnedKnowledgeWithPrerequisite(&self, pdestinationknowledge: Option<&ISyncKnowledge>) -> windows_core::Result<ISyncKnowledge>;
    fn GetLearnedForgottenKnowledge(&self) -> windows_core::Result<IForgottenKnowledge>;
}
impl windows_core::RuntimeName for ISyncChangeBatchWithPrerequisite {}
impl ISyncChangeBatchWithPrerequisite_Vtbl {
    pub const fn new<Identity: ISyncChangeBatchWithPrerequisite_Impl, const OFFSET: isize>() -> ISyncChangeBatchWithPrerequisite_Vtbl {
        unsafe extern "system" fn SetPrerequisiteKnowledge<Identity: ISyncChangeBatchWithPrerequisite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprerequisiteknowledge: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeBatchWithPrerequisite_Impl::SetPrerequisiteKnowledge(this, windows_core::from_raw_borrowed(&pprerequisiteknowledge)).into()
        }
        unsafe extern "system" fn GetLearnedKnowledgeWithPrerequisite<Identity: ISyncChangeBatchWithPrerequisite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationknowledge: *mut core::ffi::c_void, pplearnedwithprerequisiteknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeBatchWithPrerequisite_Impl::GetLearnedKnowledgeWithPrerequisite(this, windows_core::from_raw_borrowed(&pdestinationknowledge)) {
                Ok(ok__) => {
                    pplearnedwithprerequisiteknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedForgottenKnowledge<Identity: ISyncChangeBatchWithPrerequisite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeBatchWithPrerequisite_Impl::GetLearnedForgottenKnowledge(this) {
                Ok(ok__) => {
                    pplearnedforgottenknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ISyncChangeBatchBase_Vtbl::new::<Identity, OFFSET>(),
            SetPrerequisiteKnowledge: SetPrerequisiteKnowledge::<Identity, OFFSET>,
            GetLearnedKnowledgeWithPrerequisite: GetLearnedKnowledgeWithPrerequisite::<Identity, OFFSET>,
            GetLearnedForgottenKnowledge: GetLearnedForgottenKnowledge::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncChangeBatchWithPrerequisite as windows_core::Interface>::IID || iid == &<ISyncChangeBatchBase as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISyncChangeBuilder, ISyncChangeBuilder_Vtbl, 0x56f14771_8677_484f_a170_e386e418a676);
impl core::ops::Deref for ISyncChangeBuilder {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISyncChangeBuilder, windows_core::IUnknown);
impl ISyncChangeBuilder {
    pub unsafe fn AddChangeUnitMetadata(&self, pbchangeunitid: *const u8, pchangeunitversion: *const SYNC_VERSION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddChangeUnitMetadata)(windows_core::Interface::as_raw(self), pbchangeunitid, pchangeunitversion).ok()
    }
}
#[repr(C)]
pub struct ISyncChangeBuilder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddChangeUnitMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *const SYNC_VERSION) -> windows_core::HRESULT,
}
pub trait ISyncChangeBuilder_Impl: Sized + windows_core::IUnknownImpl {
    fn AddChangeUnitMetadata(&self, pbchangeunitid: *const u8, pchangeunitversion: *const SYNC_VERSION) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncChangeBuilder {}
impl ISyncChangeBuilder_Vtbl {
    pub const fn new<Identity: ISyncChangeBuilder_Impl, const OFFSET: isize>() -> ISyncChangeBuilder_Vtbl {
        unsafe extern "system" fn AddChangeUnitMetadata<Identity: ISyncChangeBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbchangeunitid: *const u8, pchangeunitversion: *const SYNC_VERSION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeBuilder_Impl::AddChangeUnitMetadata(this, core::mem::transmute_copy(&pbchangeunitid), core::mem::transmute_copy(&pchangeunitversion)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddChangeUnitMetadata: AddChangeUnitMetadata::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncChangeBuilder as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISyncChangeUnit, ISyncChangeUnit_Vtbl, 0x60edd8ca_7341_4bb7_95ce_fab6394b51cb);
impl core::ops::Deref for ISyncChangeUnit {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISyncChangeUnit, windows_core::IUnknown);
impl ISyncChangeUnit {
    pub unsafe fn GetItemChange(&self) -> windows_core::Result<ISyncChange> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetItemChange)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetChangeUnitId(&self, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetChangeUnitId)(windows_core::Interface::as_raw(self), pbchangeunitid, pcbidsize).ok()
    }
    pub unsafe fn GetChangeUnitVersion(&self, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetChangeUnitVersion)(windows_core::Interface::as_raw(self), pbcurrentreplicaid, pversion).ok()
    }
}
#[repr(C)]
pub struct ISyncChangeUnit_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetItemChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetChangeUnitId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetChangeUnitVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *mut SYNC_VERSION) -> windows_core::HRESULT,
}
pub trait ISyncChangeUnit_Impl: Sized + windows_core::IUnknownImpl {
    fn GetItemChange(&self) -> windows_core::Result<ISyncChange>;
    fn GetChangeUnitId(&self, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()>;
    fn GetChangeUnitVersion(&self, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncChangeUnit {}
impl ISyncChangeUnit_Vtbl {
    pub const fn new<Identity: ISyncChangeUnit_Impl, const OFFSET: isize>() -> ISyncChangeUnit_Vtbl {
        unsafe extern "system" fn GetItemChange<Identity: ISyncChangeUnit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsyncchange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeUnit_Impl::GetItemChange(this) {
                Ok(ok__) => {
                    ppsyncchange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChangeUnitId<Identity: ISyncChangeUnit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeUnit_Impl::GetChangeUnitId(this, core::mem::transmute_copy(&pbchangeunitid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetChangeUnitVersion<Identity: ISyncChangeUnit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeUnit_Impl::GetChangeUnitVersion(this, core::mem::transmute_copy(&pbcurrentreplicaid), core::mem::transmute_copy(&pversion)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetItemChange: GetItemChange::<Identity, OFFSET>,
            GetChangeUnitId: GetChangeUnitId::<Identity, OFFSET>,
            GetChangeUnitVersion: GetChangeUnitVersion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncChangeUnit as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISyncChangeWithFilterKeyMap, ISyncChangeWithFilterKeyMap_Vtbl, 0xbfe1ef00_e87d_42fd_a4e9_242d70414aef);
impl core::ops::Deref for ISyncChangeWithFilterKeyMap {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISyncChangeWithFilterKeyMap, windows_core::IUnknown);
impl ISyncChangeWithFilterKeyMap {
    pub unsafe fn GetFilterCount(&self, pdwfiltercount: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFilterCount)(windows_core::Interface::as_raw(self), pdwfiltercount).ok()
    }
    pub unsafe fn GetFilterChange(&self, dwfilterkey: u32, pfilterchange: *mut SYNC_FILTER_CHANGE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFilterChange)(windows_core::Interface::as_raw(self), dwfilterkey, pfilterchange).ok()
    }
    pub unsafe fn GetAllChangeUnitsPresentFlag(&self, pfallchangeunitspresent: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAllChangeUnitsPresentFlag)(windows_core::Interface::as_raw(self), pfallchangeunitspresent).ok()
    }
    pub unsafe fn GetFilterForgottenKnowledge(&self, dwfilterkey: u32) -> windows_core::Result<ISyncKnowledge> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFilterForgottenKnowledge)(windows_core::Interface::as_raw(self), dwfilterkey, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFilteredReplicaLearnedKnowledge<P0, P1>(&self, pdestinationknowledge: P0, pnewmoveins: P1) -> windows_core::Result<ISyncKnowledge>
    where
        P0: windows_core::Param<ISyncKnowledge>,
        P1: windows_core::Param<IEnumItemIds>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFilteredReplicaLearnedKnowledge)(windows_core::Interface::as_raw(self), pdestinationknowledge.param().abi(), pnewmoveins.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetLearnedFilterForgottenKnowledge<P0, P1>(&self, pdestinationknowledge: P0, pnewmoveins: P1, dwfilterkey: u32) -> windows_core::Result<ISyncKnowledge>
    where
        P0: windows_core::Param<ISyncKnowledge>,
        P1: windows_core::Param<IEnumItemIds>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLearnedFilterForgottenKnowledge)(windows_core::Interface::as_raw(self), pdestinationknowledge.param().abi(), pnewmoveins.param().abi(), dwfilterkey, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFilteredReplicaLearnedForgottenKnowledge<P0, P1>(&self, pdestinationknowledge: P0, pnewmoveins: P1) -> windows_core::Result<ISyncKnowledge>
    where
        P0: windows_core::Param<ISyncKnowledge>,
        P1: windows_core::Param<IEnumItemIds>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFilteredReplicaLearnedForgottenKnowledge)(windows_core::Interface::as_raw(self), pdestinationknowledge.param().abi(), pnewmoveins.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete<P0, P1>(&self, pdestinationknowledge: P0, pnewmoveins: P1) -> windows_core::Result<ISyncKnowledge>
    where
        P0: windows_core::Param<ISyncKnowledge>,
        P1: windows_core::Param<IEnumItemIds>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete)(windows_core::Interface::as_raw(self), pdestinationknowledge.param().abi(), pnewmoveins.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete<P0, P1>(&self, pdestinationknowledge: P0, pnewmoveins: P1, dwfilterkey: u32) -> windows_core::Result<ISyncKnowledge>
    where
        P0: windows_core::Param<ISyncKnowledge>,
        P1: windows_core::Param<IEnumItemIds>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete)(windows_core::Interface::as_raw(self), pdestinationknowledge.param().abi(), pnewmoveins.param().abi(), dwfilterkey, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISyncChangeWithFilterKeyMap_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFilterCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetFilterChange: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut SYNC_FILTER_CHANGE) -> windows_core::HRESULT,
    pub GetAllChangeUnitsPresentFlag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetFilterForgottenKnowledge: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFilteredReplicaLearnedKnowledge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLearnedFilterForgottenKnowledge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFilteredReplicaLearnedForgottenKnowledge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISyncChangeWithFilterKeyMap_Impl: Sized + windows_core::IUnknownImpl {
    fn GetFilterCount(&self, pdwfiltercount: *mut u32) -> windows_core::Result<()>;
    fn GetFilterChange(&self, dwfilterkey: u32, pfilterchange: *mut SYNC_FILTER_CHANGE) -> windows_core::Result<()>;
    fn GetAllChangeUnitsPresentFlag(&self, pfallchangeunitspresent: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetFilterForgottenKnowledge(&self, dwfilterkey: u32) -> windows_core::Result<ISyncKnowledge>;
    fn GetFilteredReplicaLearnedKnowledge(&self, pdestinationknowledge: Option<&ISyncKnowledge>, pnewmoveins: Option<&IEnumItemIds>) -> windows_core::Result<ISyncKnowledge>;
    fn GetLearnedFilterForgottenKnowledge(&self, pdestinationknowledge: Option<&ISyncKnowledge>, pnewmoveins: Option<&IEnumItemIds>, dwfilterkey: u32) -> windows_core::Result<ISyncKnowledge>;
    fn GetFilteredReplicaLearnedForgottenKnowledge(&self, pdestinationknowledge: Option<&ISyncKnowledge>, pnewmoveins: Option<&IEnumItemIds>) -> windows_core::Result<ISyncKnowledge>;
    fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete(&self, pdestinationknowledge: Option<&ISyncKnowledge>, pnewmoveins: Option<&IEnumItemIds>) -> windows_core::Result<ISyncKnowledge>;
    fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete(&self, pdestinationknowledge: Option<&ISyncKnowledge>, pnewmoveins: Option<&IEnumItemIds>, dwfilterkey: u32) -> windows_core::Result<ISyncKnowledge>;
}
impl windows_core::RuntimeName for ISyncChangeWithFilterKeyMap {}
impl ISyncChangeWithFilterKeyMap_Vtbl {
    pub const fn new<Identity: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>() -> ISyncChangeWithFilterKeyMap_Vtbl {
        unsafe extern "system" fn GetFilterCount<Identity: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwfiltercount: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeWithFilterKeyMap_Impl::GetFilterCount(this, core::mem::transmute_copy(&pdwfiltercount)).into()
        }
        unsafe extern "system" fn GetFilterChange<Identity: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfilterkey: u32, pfilterchange: *mut SYNC_FILTER_CHANGE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeWithFilterKeyMap_Impl::GetFilterChange(this, core::mem::transmute_copy(&dwfilterkey), core::mem::transmute_copy(&pfilterchange)).into()
        }
        unsafe extern "system" fn GetAllChangeUnitsPresentFlag<Identity: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfallchangeunitspresent: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeWithFilterKeyMap_Impl::GetAllChangeUnitsPresentFlag(this, core::mem::transmute_copy(&pfallchangeunitspresent)).into()
        }
        unsafe extern "system" fn GetFilterForgottenKnowledge<Identity: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfilterkey: u32, ppifilterforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeWithFilterKeyMap_Impl::GetFilterForgottenKnowledge(this, core::mem::transmute_copy(&dwfilterkey)) {
                Ok(ok__) => {
                    ppifilterforgottenknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedKnowledge<Identity: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationknowledge: *mut core::ffi::c_void, pnewmoveins: *mut core::ffi::c_void, pplearnedknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeWithFilterKeyMap_Impl::GetFilteredReplicaLearnedKnowledge(this, windows_core::from_raw_borrowed(&pdestinationknowledge), windows_core::from_raw_borrowed(&pnewmoveins)) {
                Ok(ok__) => {
                    pplearnedknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedFilterForgottenKnowledge<Identity: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationknowledge: *mut core::ffi::c_void, pnewmoveins: *mut core::ffi::c_void, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeWithFilterKeyMap_Impl::GetLearnedFilterForgottenKnowledge(this, windows_core::from_raw_borrowed(&pdestinationknowledge), windows_core::from_raw_borrowed(&pnewmoveins), core::mem::transmute_copy(&dwfilterkey)) {
                Ok(ok__) => {
                    pplearnedfilterforgottenknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedForgottenKnowledge<Identity: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationknowledge: *mut core::ffi::c_void, pnewmoveins: *mut core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeWithFilterKeyMap_Impl::GetFilteredReplicaLearnedForgottenKnowledge(this, windows_core::from_raw_borrowed(&pdestinationknowledge), windows_core::from_raw_borrowed(&pnewmoveins)) {
                Ok(ok__) => {
                    pplearnedforgottenknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete<Identity: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationknowledge: *mut core::ffi::c_void, pnewmoveins: *mut core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeWithFilterKeyMap_Impl::GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete(this, windows_core::from_raw_borrowed(&pdestinationknowledge), windows_core::from_raw_borrowed(&pnewmoveins)) {
                Ok(ok__) => {
                    pplearnedforgottenknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete<Identity: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationknowledge: *mut core::ffi::c_void, pnewmoveins: *mut core::ffi::c_void, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeWithFilterKeyMap_Impl::GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete(this, windows_core::from_raw_borrowed(&pdestinationknowledge), windows_core::from_raw_borrowed(&pnewmoveins), core::mem::transmute_copy(&dwfilterkey)) {
                Ok(ok__) => {
                    pplearnedfilterforgottenknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFilterCount: GetFilterCount::<Identity, OFFSET>,
            GetFilterChange: GetFilterChange::<Identity, OFFSET>,
            GetAllChangeUnitsPresentFlag: GetAllChangeUnitsPresentFlag::<Identity, OFFSET>,
            GetFilterForgottenKnowledge: GetFilterForgottenKnowledge::<Identity, OFFSET>,
            GetFilteredReplicaLearnedKnowledge: GetFilteredReplicaLearnedKnowledge::<Identity, OFFSET>,
            GetLearnedFilterForgottenKnowledge: GetLearnedFilterForgottenKnowledge::<Identity, OFFSET>,
            GetFilteredReplicaLearnedForgottenKnowledge: GetFilteredReplicaLearnedForgottenKnowledge::<Identity, OFFSET>,
            GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete: GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete::<Identity, OFFSET>,
            GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete: GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncChangeWithFilterKeyMap as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISyncChangeWithPrerequisite, ISyncChangeWithPrerequisite_Vtbl, 0x9e38382f_1589_48c3_92e4_05ecdcb4f3f7);
impl core::ops::Deref for ISyncChangeWithPrerequisite {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISyncChangeWithPrerequisite, windows_core::IUnknown);
impl ISyncChangeWithPrerequisite {
    pub unsafe fn GetPrerequisiteKnowledge(&self) -> windows_core::Result<ISyncKnowledge> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPrerequisiteKnowledge)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetLearnedKnowledgeWithPrerequisite<P0>(&self, pdestinationknowledge: P0) -> windows_core::Result<ISyncKnowledge>
    where
        P0: windows_core::Param<ISyncKnowledge>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLearnedKnowledgeWithPrerequisite)(windows_core::Interface::as_raw(self), pdestinationknowledge.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISyncChangeWithPrerequisite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPrerequisiteKnowledge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLearnedKnowledgeWithPrerequisite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISyncChangeWithPrerequisite_Impl: Sized + windows_core::IUnknownImpl {
    fn GetPrerequisiteKnowledge(&self) -> windows_core::Result<ISyncKnowledge>;
    fn GetLearnedKnowledgeWithPrerequisite(&self, pdestinationknowledge: Option<&ISyncKnowledge>) -> windows_core::Result<ISyncKnowledge>;
}
impl windows_core::RuntimeName for ISyncChangeWithPrerequisite {}
impl ISyncChangeWithPrerequisite_Vtbl {
    pub const fn new<Identity: ISyncChangeWithPrerequisite_Impl, const OFFSET: isize>() -> ISyncChangeWithPrerequisite_Vtbl {
        unsafe extern "system" fn GetPrerequisiteKnowledge<Identity: ISyncChangeWithPrerequisite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprerequisiteknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeWithPrerequisite_Impl::GetPrerequisiteKnowledge(this) {
                Ok(ok__) => {
                    ppprerequisiteknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedKnowledgeWithPrerequisite<Identity: ISyncChangeWithPrerequisite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationknowledge: *mut core::ffi::c_void, pplearnedknowledgewithprerequisite: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeWithPrerequisite_Impl::GetLearnedKnowledgeWithPrerequisite(this, windows_core::from_raw_borrowed(&pdestinationknowledge)) {
                Ok(ok__) => {
                    pplearnedknowledgewithprerequisite.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPrerequisiteKnowledge: GetPrerequisiteKnowledge::<Identity, OFFSET>,
            GetLearnedKnowledgeWithPrerequisite: GetLearnedKnowledgeWithPrerequisite::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncChangeWithPrerequisite as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISyncConstraintCallback, ISyncConstraintCallback_Vtbl, 0x8af3843e_75b3_438c_bb51_6f020d70d3cb);
impl core::ops::Deref for ISyncConstraintCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISyncConstraintCallback, windows_core::IUnknown);
impl ISyncConstraintCallback {
    pub unsafe fn OnConstraintConflict<P0>(&self, pconflict: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IConstraintConflict>,
    {
        (windows_core::Interface::vtable(self).OnConstraintConflict)(windows_core::Interface::as_raw(self), pconflict.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ISyncConstraintCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnConstraintConflict: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISyncConstraintCallback_Impl: Sized + windows_core::IUnknownImpl {
    fn OnConstraintConflict(&self, pconflict: Option<&IConstraintConflict>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncConstraintCallback {}
impl ISyncConstraintCallback_Vtbl {
    pub const fn new<Identity: ISyncConstraintCallback_Impl, const OFFSET: isize>() -> ISyncConstraintCallback_Vtbl {
        unsafe extern "system" fn OnConstraintConflict<Identity: ISyncConstraintCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconflict: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncConstraintCallback_Impl::OnConstraintConflict(this, windows_core::from_raw_borrowed(&pconflict)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnConstraintConflict: OnConstraintConflict::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncConstraintCallback as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISyncDataConverter, ISyncDataConverter_Vtbl, 0x435d4861_68d5_44aa_a0f9_72a0b00ef9cf);
impl core::ops::Deref for ISyncDataConverter {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISyncDataConverter, windows_core::IUnknown);
impl ISyncDataConverter {
    pub unsafe fn ConvertDataRetrieverFromProviderFormat<P0, P1>(&self, punkdataretrieverin: P0, penumsyncchanges: P1) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<IEnumSyncChanges>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ConvertDataRetrieverFromProviderFormat)(windows_core::Interface::as_raw(self), punkdataretrieverin.param().abi(), penumsyncchanges.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ConvertDataRetrieverToProviderFormat<P0, P1>(&self, punkdataretrieverin: P0, penumsyncchanges: P1) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<IEnumSyncChanges>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ConvertDataRetrieverToProviderFormat)(windows_core::Interface::as_raw(self), punkdataretrieverin.param().abi(), penumsyncchanges.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ConvertDataFromProviderFormat<P0, P1>(&self, pdatacontext: P0, punkdatain: P1) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<ILoadChangeContext>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ConvertDataFromProviderFormat)(windows_core::Interface::as_raw(self), pdatacontext.param().abi(), punkdatain.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ConvertDataToProviderFormat<P0, P1>(&self, pdatacontext: P0, punkdataout: P1) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<ILoadChangeContext>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ConvertDataToProviderFormat)(windows_core::Interface::as_raw(self), pdatacontext.param().abi(), punkdataout.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISyncDataConverter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ConvertDataRetrieverFromProviderFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConvertDataRetrieverToProviderFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConvertDataFromProviderFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConvertDataToProviderFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISyncDataConverter_Impl: Sized + windows_core::IUnknownImpl {
    fn ConvertDataRetrieverFromProviderFormat(&self, punkdataretrieverin: Option<&windows_core::IUnknown>, penumsyncchanges: Option<&IEnumSyncChanges>) -> windows_core::Result<windows_core::IUnknown>;
    fn ConvertDataRetrieverToProviderFormat(&self, punkdataretrieverin: Option<&windows_core::IUnknown>, penumsyncchanges: Option<&IEnumSyncChanges>) -> windows_core::Result<windows_core::IUnknown>;
    fn ConvertDataFromProviderFormat(&self, pdatacontext: Option<&ILoadChangeContext>, punkdatain: Option<&windows_core::IUnknown>) -> windows_core::Result<windows_core::IUnknown>;
    fn ConvertDataToProviderFormat(&self, pdatacontext: Option<&ILoadChangeContext>, punkdataout: Option<&windows_core::IUnknown>) -> windows_core::Result<windows_core::IUnknown>;
}
impl windows_core::RuntimeName for ISyncDataConverter {}
impl ISyncDataConverter_Vtbl {
    pub const fn new<Identity: ISyncDataConverter_Impl, const OFFSET: isize>() -> ISyncDataConverter_Vtbl {
        unsafe extern "system" fn ConvertDataRetrieverFromProviderFormat<Identity: ISyncDataConverter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkdataretrieverin: *mut core::ffi::c_void, penumsyncchanges: *mut core::ffi::c_void, ppunkdataout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncDataConverter_Impl::ConvertDataRetrieverFromProviderFormat(this, windows_core::from_raw_borrowed(&punkdataretrieverin), windows_core::from_raw_borrowed(&penumsyncchanges)) {
                Ok(ok__) => {
                    ppunkdataout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertDataRetrieverToProviderFormat<Identity: ISyncDataConverter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkdataretrieverin: *mut core::ffi::c_void, penumsyncchanges: *mut core::ffi::c_void, ppunkdataout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncDataConverter_Impl::ConvertDataRetrieverToProviderFormat(this, windows_core::from_raw_borrowed(&punkdataretrieverin), windows_core::from_raw_borrowed(&penumsyncchanges)) {
                Ok(ok__) => {
                    ppunkdataout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertDataFromProviderFormat<Identity: ISyncDataConverter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatacontext: *mut core::ffi::c_void, punkdatain: *mut core::ffi::c_void, ppunkdataout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncDataConverter_Impl::ConvertDataFromProviderFormat(this, windows_core::from_raw_borrowed(&pdatacontext), windows_core::from_raw_borrowed(&punkdatain)) {
                Ok(ok__) => {
                    ppunkdataout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertDataToProviderFormat<Identity: ISyncDataConverter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatacontext: *mut core::ffi::c_void, punkdataout: *mut core::ffi::c_void, ppunkdataout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncDataConverter_Impl::ConvertDataToProviderFormat(this, windows_core::from_raw_borrowed(&pdatacontext), windows_core::from_raw_borrowed(&punkdataout)) {
                Ok(ok__) => {
                    ppunkdataout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ConvertDataRetrieverFromProviderFormat: ConvertDataRetrieverFromProviderFormat::<Identity, OFFSET>,
            ConvertDataRetrieverToProviderFormat: ConvertDataRetrieverToProviderFormat::<Identity, OFFSET>,
            ConvertDataFromProviderFormat: ConvertDataFromProviderFormat::<Identity, OFFSET>,
            ConvertDataToProviderFormat: ConvertDataToProviderFormat::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncDataConverter as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISyncFilter, ISyncFilter_Vtbl, 0x087a3f15_0fcb_44c1_9639_53c14e2b5506);
impl core::ops::Deref for ISyncFilter {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISyncFilter, windows_core::IUnknown);
impl ISyncFilter {
    pub unsafe fn IsIdentical<P0>(&self, psyncfilter: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISyncFilter>,
    {
        (windows_core::Interface::vtable(self).IsIdentical)(windows_core::Interface::as_raw(self), psyncfilter.param().abi()).ok()
    }
    pub unsafe fn Serialize(&self, pbsyncfilter: *mut u8, pcbsyncfilter: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Serialize)(windows_core::Interface::as_raw(self), pbsyncfilter, pcbsyncfilter).ok()
    }
}
#[repr(C)]
pub struct ISyncFilter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsIdentical: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Serialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
}
pub trait ISyncFilter_Impl: Sized + windows_core::IUnknownImpl {
    fn IsIdentical(&self, psyncfilter: Option<&ISyncFilter>) -> windows_core::Result<()>;
    fn Serialize(&self, pbsyncfilter: *mut u8, pcbsyncfilter: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncFilter {}
impl ISyncFilter_Vtbl {
    pub const fn new<Identity: ISyncFilter_Impl, const OFFSET: isize>() -> ISyncFilter_Vtbl {
        unsafe extern "system" fn IsIdentical<Identity: ISyncFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psyncfilter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncFilter_Impl::IsIdentical(this, windows_core::from_raw_borrowed(&psyncfilter)).into()
        }
        unsafe extern "system" fn Serialize<Identity: ISyncFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsyncfilter: *mut u8, pcbsyncfilter: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncFilter_Impl::Serialize(this, core::mem::transmute_copy(&pbsyncfilter), core::mem::transmute_copy(&pcbsyncfilter)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsIdentical: IsIdentical::<Identity, OFFSET>,
            Serialize: Serialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncFilter as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISyncFilterDeserializer, ISyncFilterDeserializer_Vtbl, 0xb45b7a72_e5c7_46be_9c82_77b8b15dab8a);
impl core::ops::Deref for ISyncFilterDeserializer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISyncFilterDeserializer, windows_core::IUnknown);
impl ISyncFilterDeserializer {
    pub unsafe fn DeserializeSyncFilter(&self, pbsyncfilter: *const u8, dwcbsyncfilter: u32) -> windows_core::Result<ISyncFilter> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DeserializeSyncFilter)(windows_core::Interface::as_raw(self), pbsyncfilter, dwcbsyncfilter, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISyncFilterDeserializer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DeserializeSyncFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISyncFilterDeserializer_Impl: Sized + windows_core::IUnknownImpl {
    fn DeserializeSyncFilter(&self, pbsyncfilter: *const u8, dwcbsyncfilter: u32) -> windows_core::Result<ISyncFilter>;
}
impl windows_core::RuntimeName for ISyncFilterDeserializer {}
impl ISyncFilterDeserializer_Vtbl {
    pub const fn new<Identity: ISyncFilterDeserializer_Impl, const OFFSET: isize>() -> ISyncFilterDeserializer_Vtbl {
        unsafe extern "system" fn DeserializeSyncFilter<Identity: ISyncFilterDeserializer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsyncfilter: *const u8, dwcbsyncfilter: u32, ppisyncfilter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncFilterDeserializer_Impl::DeserializeSyncFilter(this, core::mem::transmute_copy(&pbsyncfilter), core::mem::transmute_copy(&dwcbsyncfilter)) {
                Ok(ok__) => {
                    ppisyncfilter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), DeserializeSyncFilter: DeserializeSyncFilter::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncFilterDeserializer as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISyncFilterInfo, ISyncFilterInfo_Vtbl, 0x794eaaf8_3f2e_47e6_9728_17e6fcf94cb7);
impl core::ops::Deref for ISyncFilterInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISyncFilterInfo, windows_core::IUnknown);
impl ISyncFilterInfo {
    pub unsafe fn Serialize(&self, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Serialize)(windows_core::Interface::as_raw(self), pbbuffer, pcbbuffer).ok()
    }
}
#[repr(C)]
pub struct ISyncFilterInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Serialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
}
pub trait ISyncFilterInfo_Impl: Sized + windows_core::IUnknownImpl {
    fn Serialize(&self, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncFilterInfo {}
impl ISyncFilterInfo_Vtbl {
    pub const fn new<Identity: ISyncFilterInfo_Impl, const OFFSET: isize>() -> ISyncFilterInfo_Vtbl {
        unsafe extern "system" fn Serialize<Identity: ISyncFilterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncFilterInfo_Impl::Serialize(this, core::mem::transmute_copy(&pbbuffer), core::mem::transmute_copy(&pcbbuffer)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Serialize: Serialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncFilterInfo as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISyncFilterInfo2, ISyncFilterInfo2_Vtbl, 0x19b394ba_e3d0_468c_934d_321968b2ab34);
impl core::ops::Deref for ISyncFilterInfo2 {
    type Target = ISyncFilterInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISyncFilterInfo2, windows_core::IUnknown, ISyncFilterInfo);
impl ISyncFilterInfo2 {
    pub unsafe fn GetFlags(&self, pdwflags: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFlags)(windows_core::Interface::as_raw(self), pdwflags).ok()
    }
}
#[repr(C)]
pub struct ISyncFilterInfo2_Vtbl {
    pub base__: ISyncFilterInfo_Vtbl,
    pub GetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait ISyncFilterInfo2_Impl: Sized + ISyncFilterInfo_Impl {
    fn GetFlags(&self, pdwflags: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncFilterInfo2 {}
impl ISyncFilterInfo2_Vtbl {
    pub const fn new<Identity: ISyncFilterInfo2_Impl, const OFFSET: isize>() -> ISyncFilterInfo2_Vtbl {
        unsafe extern "system" fn GetFlags<Identity: ISyncFilterInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncFilterInfo2_Impl::GetFlags(this, core::mem::transmute_copy(&pdwflags)).into()
        }
        Self { base__: ISyncFilterInfo_Vtbl::new::<Identity, OFFSET>(), GetFlags: GetFlags::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncFilterInfo2 as windows_core::Interface>::IID || iid == &<ISyncFilterInfo as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISyncFullEnumerationChange, ISyncFullEnumerationChange_Vtbl, 0x9785e0bd_bdff_40c4_98c5_b34b2f1991b3);
impl core::ops::Deref for ISyncFullEnumerationChange {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISyncFullEnumerationChange, windows_core::IUnknown);
impl ISyncFullEnumerationChange {
    pub unsafe fn GetLearnedKnowledgeAfterRecoveryComplete(&self) -> windows_core::Result<ISyncKnowledge> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLearnedKnowledgeAfterRecoveryComplete)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetLearnedForgottenKnowledge(&self) -> windows_core::Result<IForgottenKnowledge> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLearnedForgottenKnowledge)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISyncFullEnumerationChange_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLearnedKnowledgeAfterRecoveryComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLearnedForgottenKnowledge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISyncFullEnumerationChange_Impl: Sized + windows_core::IUnknownImpl {
    fn GetLearnedKnowledgeAfterRecoveryComplete(&self) -> windows_core::Result<ISyncKnowledge>;
    fn GetLearnedForgottenKnowledge(&self) -> windows_core::Result<IForgottenKnowledge>;
}
impl windows_core::RuntimeName for ISyncFullEnumerationChange {}
impl ISyncFullEnumerationChange_Vtbl {
    pub const fn new<Identity: ISyncFullEnumerationChange_Impl, const OFFSET: isize>() -> ISyncFullEnumerationChange_Vtbl {
        unsafe extern "system" fn GetLearnedKnowledgeAfterRecoveryComplete<Identity: ISyncFullEnumerationChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplearnedknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncFullEnumerationChange_Impl::GetLearnedKnowledgeAfterRecoveryComplete(this) {
                Ok(ok__) => {
                    pplearnedknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedForgottenKnowledge<Identity: ISyncFullEnumerationChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncFullEnumerationChange_Impl::GetLearnedForgottenKnowledge(this) {
                Ok(ok__) => {
                    pplearnedforgottenknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLearnedKnowledgeAfterRecoveryComplete: GetLearnedKnowledgeAfterRecoveryComplete::<Identity, OFFSET>,
            GetLearnedForgottenKnowledge: GetLearnedForgottenKnowledge::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncFullEnumerationChange as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISyncFullEnumerationChangeBatch, ISyncFullEnumerationChangeBatch_Vtbl, 0xef64197d_4f44_4ea2_b355_4524713e3bed);
impl core::ops::Deref for ISyncFullEnumerationChangeBatch {
    type Target = ISyncChangeBatchBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISyncFullEnumerationChangeBatch, windows_core::IUnknown, ISyncChangeBatchBase);
impl ISyncFullEnumerationChangeBatch {
    pub unsafe fn GetLearnedKnowledgeAfterRecoveryComplete(&self) -> windows_core::Result<ISyncKnowledge> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLearnedKnowledgeAfterRecoveryComplete)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetClosedLowerBoundItemId(&self, pbclosedlowerbounditemid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetClosedLowerBoundItemId)(windows_core::Interface::as_raw(self), pbclosedlowerbounditemid, pcbidsize).ok()
    }
    pub unsafe fn GetClosedUpperBoundItemId(&self, pbclosedupperbounditemid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetClosedUpperBoundItemId)(windows_core::Interface::as_raw(self), pbclosedupperbounditemid, pcbidsize).ok()
    }
}
#[repr(C)]
pub struct ISyncFullEnumerationChangeBatch_Vtbl {
    pub base__: ISyncChangeBatchBase_Vtbl,
    pub GetLearnedKnowledgeAfterRecoveryComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetClosedLowerBoundItemId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetClosedUpperBoundItemId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
}
pub trait ISyncFullEnumerationChangeBatch_Impl: Sized + ISyncChangeBatchBase_Impl {
    fn GetLearnedKnowledgeAfterRecoveryComplete(&self) -> windows_core::Result<ISyncKnowledge>;
    fn GetClosedLowerBoundItemId(&self, pbclosedlowerbounditemid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()>;
    fn GetClosedUpperBoundItemId(&self, pbclosedupperbounditemid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncFullEnumerationChangeBatch {}
impl ISyncFullEnumerationChangeBatch_Vtbl {
    pub const fn new<Identity: ISyncFullEnumerationChangeBatch_Impl, const OFFSET: isize>() -> ISyncFullEnumerationChangeBatch_Vtbl {
        unsafe extern "system" fn GetLearnedKnowledgeAfterRecoveryComplete<Identity: ISyncFullEnumerationChangeBatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplearnedknowledgeafterrecoverycomplete: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncFullEnumerationChangeBatch_Impl::GetLearnedKnowledgeAfterRecoveryComplete(this) {
                Ok(ok__) => {
                    pplearnedknowledgeafterrecoverycomplete.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClosedLowerBoundItemId<Identity: ISyncFullEnumerationChangeBatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbclosedlowerbounditemid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncFullEnumerationChangeBatch_Impl::GetClosedLowerBoundItemId(this, core::mem::transmute_copy(&pbclosedlowerbounditemid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetClosedUpperBoundItemId<Identity: ISyncFullEnumerationChangeBatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbclosedupperbounditemid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncFullEnumerationChangeBatch_Impl::GetClosedUpperBoundItemId(this, core::mem::transmute_copy(&pbclosedupperbounditemid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        Self {
            base__: ISyncChangeBatchBase_Vtbl::new::<Identity, OFFSET>(),
            GetLearnedKnowledgeAfterRecoveryComplete: GetLearnedKnowledgeAfterRecoveryComplete::<Identity, OFFSET>,
            GetClosedLowerBoundItemId: GetClosedLowerBoundItemId::<Identity, OFFSET>,
            GetClosedUpperBoundItemId: GetClosedUpperBoundItemId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncFullEnumerationChangeBatch as windows_core::Interface>::IID || iid == &<ISyncChangeBatchBase as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISyncFullEnumerationChangeBatch2, ISyncFullEnumerationChangeBatch2_Vtbl, 0xe06449f4_a205_4b65_9724_01b22101eec1);
impl core::ops::Deref for ISyncFullEnumerationChangeBatch2 {
    type Target = ISyncFullEnumerationChangeBatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISyncFullEnumerationChangeBatch2, windows_core::IUnknown, ISyncChangeBatchBase, ISyncFullEnumerationChangeBatch);
impl ISyncFullEnumerationChangeBatch2 {
    pub unsafe fn AddMergeTombstoneMetadataToGroup(&self, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32) -> windows_core::Result<ISyncChangeBuilder> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddMergeTombstoneMetadataToGroup)(windows_core::Interface::as_raw(self), pbownerreplicaid, pbwinneritemid, pbitemid, pchangeversion, pcreationversion, dwworkforchange, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISyncFullEnumerationChangeBatch2_Vtbl {
    pub base__: ISyncFullEnumerationChangeBatch_Vtbl,
    pub AddMergeTombstoneMetadataToGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *const u8, *const u8, *const SYNC_VERSION, *const SYNC_VERSION, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISyncFullEnumerationChangeBatch2_Impl: Sized + ISyncFullEnumerationChangeBatch_Impl {
    fn AddMergeTombstoneMetadataToGroup(&self, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32) -> windows_core::Result<ISyncChangeBuilder>;
}
impl windows_core::RuntimeName for ISyncFullEnumerationChangeBatch2 {}
impl ISyncFullEnumerationChangeBatch2_Vtbl {
    pub const fn new<Identity: ISyncFullEnumerationChangeBatch2_Impl, const OFFSET: isize>() -> ISyncFullEnumerationChangeBatch2_Vtbl {
        unsafe extern "system" fn AddMergeTombstoneMetadataToGroup<Identity: ISyncFullEnumerationChangeBatch2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, ppchangebuilder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncFullEnumerationChangeBatch2_Impl::AddMergeTombstoneMetadataToGroup(this, core::mem::transmute_copy(&pbownerreplicaid), core::mem::transmute_copy(&pbwinneritemid), core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pchangeversion), core::mem::transmute_copy(&pcreationversion), core::mem::transmute_copy(&dwworkforchange)) {
                Ok(ok__) => {
                    ppchangebuilder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ISyncFullEnumerationChangeBatch_Vtbl::new::<Identity, OFFSET>(),
            AddMergeTombstoneMetadataToGroup: AddMergeTombstoneMetadataToGroup::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncFullEnumerationChangeBatch2 as windows_core::Interface>::IID || iid == &<ISyncChangeBatchBase as windows_core::Interface>::IID || iid == &<ISyncFullEnumerationChangeBatch as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISyncKnowledge, ISyncKnowledge_Vtbl, 0x615bbb53_c945_4203_bf4b_2cb65919a0aa);
impl core::ops::Deref for ISyncKnowledge {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISyncKnowledge, windows_core::IUnknown);
impl ISyncKnowledge {
    pub unsafe fn GetOwnerReplicaId(&self, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetOwnerReplicaId)(windows_core::Interface::as_raw(self), pbreplicaid, pcbidsize).ok()
    }
    pub unsafe fn Serialize<P0>(&self, fserializereplicakeymap: P0, pbknowledge: *mut u8, pcbknowledge: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Serialize)(windows_core::Interface::as_raw(self), fserializereplicakeymap.param().abi(), pbknowledge, pcbknowledge).ok()
    }
    pub unsafe fn SetLocalTickCount(&self, ulltickcount: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLocalTickCount)(windows_core::Interface::as_raw(self), ulltickcount).ok()
    }
    pub unsafe fn ContainsChange(&self, pbversionownerreplicaid: *const u8, pgiditemid: *const u8, psyncversion: *const SYNC_VERSION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ContainsChange)(windows_core::Interface::as_raw(self), pbversionownerreplicaid, pgiditemid, psyncversion).ok()
    }
    pub unsafe fn ContainsChangeUnit(&self, pbversionownerreplicaid: *const u8, pbitemid: *const u8, pbchangeunitid: *const u8, psyncversion: *const SYNC_VERSION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ContainsChangeUnit)(windows_core::Interface::as_raw(self), pbversionownerreplicaid, pbitemid, pbchangeunitid, psyncversion).ok()
    }
    pub unsafe fn GetScopeVector(&self, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetScopeVector)(windows_core::Interface::as_raw(self), riid, ppunk).ok()
    }
    pub unsafe fn GetReplicaKeyMap(&self) -> windows_core::Result<IReplicaKeyMap> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetReplicaKeyMap)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<ISyncKnowledge> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ConvertVersion<P0>(&self, pknowledgein: P0, pbcurrentownerid: *const u8, pversionin: *const SYNC_VERSION, pbnewownerid: *mut u8, pcbidsize: *mut u32, pversionout: *mut SYNC_VERSION) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISyncKnowledge>,
    {
        (windows_core::Interface::vtable(self).ConvertVersion)(windows_core::Interface::as_raw(self), pknowledgein.param().abi(), pbcurrentownerid, pversionin, pbnewownerid, pcbidsize, pversionout).ok()
    }
    pub unsafe fn MapRemoteToLocal<P0>(&self, premoteknowledge: P0) -> windows_core::Result<ISyncKnowledge>
    where
        P0: windows_core::Param<ISyncKnowledge>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MapRemoteToLocal)(windows_core::Interface::as_raw(self), premoteknowledge.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Union<P0>(&self, pknowledge: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISyncKnowledge>,
    {
        (windows_core::Interface::vtable(self).Union)(windows_core::Interface::as_raw(self), pknowledge.param().abi()).ok()
    }
    pub unsafe fn ProjectOntoItem(&self, pbitemid: *const u8) -> windows_core::Result<ISyncKnowledge> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProjectOntoItem)(windows_core::Interface::as_raw(self), pbitemid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ProjectOntoChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8) -> windows_core::Result<ISyncKnowledge> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProjectOntoChangeUnit)(windows_core::Interface::as_raw(self), pbitemid, pbchangeunitid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ProjectOntoRange(&self, psrngsyncrange: *const SYNC_RANGE) -> windows_core::Result<ISyncKnowledge> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProjectOntoRange)(windows_core::Interface::as_raw(self), psrngsyncrange, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ExcludeItem(&self, pbitemid: *const u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ExcludeItem)(windows_core::Interface::as_raw(self), pbitemid).ok()
    }
    pub unsafe fn ExcludeChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ExcludeChangeUnit)(windows_core::Interface::as_raw(self), pbitemid, pbchangeunitid).ok()
    }
    pub unsafe fn ContainsKnowledge<P0>(&self, pknowledge: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISyncKnowledge>,
    {
        (windows_core::Interface::vtable(self).ContainsKnowledge)(windows_core::Interface::as_raw(self), pknowledge.param().abi()).ok()
    }
    pub unsafe fn FindMinTickCountForReplica(&self, pbreplicaid: *const u8, pullreplicatickcount: *mut u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FindMinTickCountForReplica)(windows_core::Interface::as_raw(self), pbreplicaid, pullreplicatickcount).ok()
    }
    pub unsafe fn GetRangeExceptions(&self, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRangeExceptions)(windows_core::Interface::as_raw(self), riid, ppunk).ok()
    }
    pub unsafe fn GetSingleItemExceptions(&self, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSingleItemExceptions)(windows_core::Interface::as_raw(self), riid, ppunk).ok()
    }
    pub unsafe fn GetChangeUnitExceptions(&self, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetChangeUnitExceptions)(windows_core::Interface::as_raw(self), riid, ppunk).ok()
    }
    pub unsafe fn FindClockVectorForItem(&self, pbitemid: *const u8, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FindClockVectorForItem)(windows_core::Interface::as_raw(self), pbitemid, riid, ppunk).ok()
    }
    pub unsafe fn FindClockVectorForChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FindClockVectorForChangeUnit)(windows_core::Interface::as_raw(self), pbitemid, pbchangeunitid, riid, ppunk).ok()
    }
    pub unsafe fn GetVersion(&self, pdwversion: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetVersion)(windows_core::Interface::as_raw(self), pdwversion).ok()
    }
}
#[repr(C)]
pub struct ISyncKnowledge_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetOwnerReplicaId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub Serialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub SetLocalTickCount: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub ContainsChange: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *const u8, *const SYNC_VERSION) -> windows_core::HRESULT,
    pub ContainsChangeUnit: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *const u8, *const u8, *const SYNC_VERSION) -> windows_core::HRESULT,
    pub GetScopeVector: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetReplicaKeyMap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConvertVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const u8, *const SYNC_VERSION, *mut u8, *mut u32, *mut SYNC_VERSION) -> windows_core::HRESULT,
    pub MapRemoteToLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Union: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProjectOntoItem: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProjectOntoChangeUnit: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProjectOntoRange: unsafe extern "system" fn(*mut core::ffi::c_void, *const SYNC_RANGE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExcludeItem: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8) -> windows_core::HRESULT,
    pub ExcludeChangeUnit: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *const u8) -> windows_core::HRESULT,
    pub ContainsKnowledge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindMinTickCountForReplica: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *mut u64) -> windows_core::HRESULT,
    pub GetRangeExceptions: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSingleItemExceptions: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetChangeUnitExceptions: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindClockVectorForItem: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindClockVectorForChangeUnit: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *const u8, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait ISyncKnowledge_Impl: Sized + windows_core::IUnknownImpl {
    fn GetOwnerReplicaId(&self, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()>;
    fn Serialize(&self, fserializereplicakeymap: super::super::Foundation::BOOL, pbknowledge: *mut u8, pcbknowledge: *mut u32) -> windows_core::Result<()>;
    fn SetLocalTickCount(&self, ulltickcount: u64) -> windows_core::Result<()>;
    fn ContainsChange(&self, pbversionownerreplicaid: *const u8, pgiditemid: *const u8, psyncversion: *const SYNC_VERSION) -> windows_core::Result<()>;
    fn ContainsChangeUnit(&self, pbversionownerreplicaid: *const u8, pbitemid: *const u8, pbchangeunitid: *const u8, psyncversion: *const SYNC_VERSION) -> windows_core::Result<()>;
    fn GetScopeVector(&self, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetReplicaKeyMap(&self) -> windows_core::Result<IReplicaKeyMap>;
    fn Clone(&self) -> windows_core::Result<ISyncKnowledge>;
    fn ConvertVersion(&self, pknowledgein: Option<&ISyncKnowledge>, pbcurrentownerid: *const u8, pversionin: *const SYNC_VERSION, pbnewownerid: *mut u8, pcbidsize: *mut u32, pversionout: *mut SYNC_VERSION) -> windows_core::Result<()>;
    fn MapRemoteToLocal(&self, premoteknowledge: Option<&ISyncKnowledge>) -> windows_core::Result<ISyncKnowledge>;
    fn Union(&self, pknowledge: Option<&ISyncKnowledge>) -> windows_core::Result<()>;
    fn ProjectOntoItem(&self, pbitemid: *const u8) -> windows_core::Result<ISyncKnowledge>;
    fn ProjectOntoChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8) -> windows_core::Result<ISyncKnowledge>;
    fn ProjectOntoRange(&self, psrngsyncrange: *const SYNC_RANGE) -> windows_core::Result<ISyncKnowledge>;
    fn ExcludeItem(&self, pbitemid: *const u8) -> windows_core::Result<()>;
    fn ExcludeChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8) -> windows_core::Result<()>;
    fn ContainsKnowledge(&self, pknowledge: Option<&ISyncKnowledge>) -> windows_core::Result<()>;
    fn FindMinTickCountForReplica(&self, pbreplicaid: *const u8, pullreplicatickcount: *mut u64) -> windows_core::Result<()>;
    fn GetRangeExceptions(&self, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetSingleItemExceptions(&self, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetChangeUnitExceptions(&self, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn FindClockVectorForItem(&self, pbitemid: *const u8, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn FindClockVectorForChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetVersion(&self, pdwversion: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncKnowledge {}
impl ISyncKnowledge_Vtbl {
    pub const fn new<Identity: ISyncKnowledge_Impl, const OFFSET: isize>() -> ISyncKnowledge_Vtbl {
        unsafe extern "system" fn GetOwnerReplicaId<Identity: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge_Impl::GetOwnerReplicaId(this, core::mem::transmute_copy(&pbreplicaid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn Serialize<Identity: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fserializereplicakeymap: super::super::Foundation::BOOL, pbknowledge: *mut u8, pcbknowledge: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge_Impl::Serialize(this, core::mem::transmute_copy(&fserializereplicakeymap), core::mem::transmute_copy(&pbknowledge), core::mem::transmute_copy(&pcbknowledge)).into()
        }
        unsafe extern "system" fn SetLocalTickCount<Identity: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulltickcount: u64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge_Impl::SetLocalTickCount(this, core::mem::transmute_copy(&ulltickcount)).into()
        }
        unsafe extern "system" fn ContainsChange<Identity: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbversionownerreplicaid: *const u8, pgiditemid: *const u8, psyncversion: *const SYNC_VERSION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge_Impl::ContainsChange(this, core::mem::transmute_copy(&pbversionownerreplicaid), core::mem::transmute_copy(&pgiditemid), core::mem::transmute_copy(&psyncversion)).into()
        }
        unsafe extern "system" fn ContainsChangeUnit<Identity: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbversionownerreplicaid: *const u8, pbitemid: *const u8, pbchangeunitid: *const u8, psyncversion: *const SYNC_VERSION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge_Impl::ContainsChangeUnit(this, core::mem::transmute_copy(&pbversionownerreplicaid), core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pbchangeunitid), core::mem::transmute_copy(&psyncversion)).into()
        }
        unsafe extern "system" fn GetScopeVector<Identity: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge_Impl::GetScopeVector(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn GetReplicaKeyMap<Identity: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppreplicakeymap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncKnowledge_Impl::GetReplicaKeyMap(this) {
                Ok(ok__) => {
                    ppreplicakeymap.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclonedknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncKnowledge_Impl::Clone(this) {
                Ok(ok__) => {
                    ppclonedknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertVersion<Identity: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pknowledgein: *mut core::ffi::c_void, pbcurrentownerid: *const u8, pversionin: *const SYNC_VERSION, pbnewownerid: *mut u8, pcbidsize: *mut u32, pversionout: *mut SYNC_VERSION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge_Impl::ConvertVersion(this, windows_core::from_raw_borrowed(&pknowledgein), core::mem::transmute_copy(&pbcurrentownerid), core::mem::transmute_copy(&pversionin), core::mem::transmute_copy(&pbnewownerid), core::mem::transmute_copy(&pcbidsize), core::mem::transmute_copy(&pversionout)).into()
        }
        unsafe extern "system" fn MapRemoteToLocal<Identity: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, premoteknowledge: *mut core::ffi::c_void, ppmappedknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncKnowledge_Impl::MapRemoteToLocal(this, windows_core::from_raw_borrowed(&premoteknowledge)) {
                Ok(ok__) => {
                    ppmappedknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Union<Identity: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pknowledge: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge_Impl::Union(this, windows_core::from_raw_borrowed(&pknowledge)).into()
        }
        unsafe extern "system" fn ProjectOntoItem<Identity: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitemid: *const u8, ppknowledgeout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncKnowledge_Impl::ProjectOntoItem(this, core::mem::transmute_copy(&pbitemid)) {
                Ok(ok__) => {
                    ppknowledgeout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProjectOntoChangeUnit<Identity: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8, ppknowledgeout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncKnowledge_Impl::ProjectOntoChangeUnit(this, core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pbchangeunitid)) {
                Ok(ok__) => {
                    ppknowledgeout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProjectOntoRange<Identity: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrngsyncrange: *const SYNC_RANGE, ppknowledgeout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncKnowledge_Impl::ProjectOntoRange(this, core::mem::transmute_copy(&psrngsyncrange)) {
                Ok(ok__) => {
                    ppknowledgeout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExcludeItem<Identity: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitemid: *const u8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge_Impl::ExcludeItem(this, core::mem::transmute_copy(&pbitemid)).into()
        }
        unsafe extern "system" fn ExcludeChangeUnit<Identity: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge_Impl::ExcludeChangeUnit(this, core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pbchangeunitid)).into()
        }
        unsafe extern "system" fn ContainsKnowledge<Identity: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pknowledge: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge_Impl::ContainsKnowledge(this, windows_core::from_raw_borrowed(&pknowledge)).into()
        }
        unsafe extern "system" fn FindMinTickCountForReplica<Identity: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbreplicaid: *const u8, pullreplicatickcount: *mut u64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge_Impl::FindMinTickCountForReplica(this, core::mem::transmute_copy(&pbreplicaid), core::mem::transmute_copy(&pullreplicatickcount)).into()
        }
        unsafe extern "system" fn GetRangeExceptions<Identity: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge_Impl::GetRangeExceptions(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn GetSingleItemExceptions<Identity: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge_Impl::GetSingleItemExceptions(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn GetChangeUnitExceptions<Identity: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge_Impl::GetChangeUnitExceptions(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn FindClockVectorForItem<Identity: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitemid: *const u8, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge_Impl::FindClockVectorForItem(this, core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn FindClockVectorForChangeUnit<Identity: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge_Impl::FindClockVectorForChangeUnit(this, core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pbchangeunitid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn GetVersion<Identity: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwversion: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge_Impl::GetVersion(this, core::mem::transmute_copy(&pdwversion)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOwnerReplicaId: GetOwnerReplicaId::<Identity, OFFSET>,
            Serialize: Serialize::<Identity, OFFSET>,
            SetLocalTickCount: SetLocalTickCount::<Identity, OFFSET>,
            ContainsChange: ContainsChange::<Identity, OFFSET>,
            ContainsChangeUnit: ContainsChangeUnit::<Identity, OFFSET>,
            GetScopeVector: GetScopeVector::<Identity, OFFSET>,
            GetReplicaKeyMap: GetReplicaKeyMap::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            ConvertVersion: ConvertVersion::<Identity, OFFSET>,
            MapRemoteToLocal: MapRemoteToLocal::<Identity, OFFSET>,
            Union: Union::<Identity, OFFSET>,
            ProjectOntoItem: ProjectOntoItem::<Identity, OFFSET>,
            ProjectOntoChangeUnit: ProjectOntoChangeUnit::<Identity, OFFSET>,
            ProjectOntoRange: ProjectOntoRange::<Identity, OFFSET>,
            ExcludeItem: ExcludeItem::<Identity, OFFSET>,
            ExcludeChangeUnit: ExcludeChangeUnit::<Identity, OFFSET>,
            ContainsKnowledge: ContainsKnowledge::<Identity, OFFSET>,
            FindMinTickCountForReplica: FindMinTickCountForReplica::<Identity, OFFSET>,
            GetRangeExceptions: GetRangeExceptions::<Identity, OFFSET>,
            GetSingleItemExceptions: GetSingleItemExceptions::<Identity, OFFSET>,
            GetChangeUnitExceptions: GetChangeUnitExceptions::<Identity, OFFSET>,
            FindClockVectorForItem: FindClockVectorForItem::<Identity, OFFSET>,
            FindClockVectorForChangeUnit: FindClockVectorForChangeUnit::<Identity, OFFSET>,
            GetVersion: GetVersion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncKnowledge as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISyncKnowledge2, ISyncKnowledge2_Vtbl, 0xed0addc0_3b4b_46a1_9a45_45661d2114c8);
impl core::ops::Deref for ISyncKnowledge2 {
    type Target = ISyncKnowledge;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISyncKnowledge2, windows_core::IUnknown, ISyncKnowledge);
impl ISyncKnowledge2 {
    pub unsafe fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetIdParameters)(windows_core::Interface::as_raw(self), pidparameters).ok()
    }
    pub unsafe fn ProjectOntoColumnSet(&self, ppcolumns: *const *const u8, count: u32) -> windows_core::Result<ISyncKnowledge2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProjectOntoColumnSet)(windows_core::Interface::as_raw(self), ppcolumns, count, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SerializeWithOptions(&self, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SerializeWithOptions)(windows_core::Interface::as_raw(self), targetformatversion, dwflags, pbbuffer, pdwserializedsize).ok()
    }
    pub unsafe fn GetLowestUncontainedId<P0>(&self, pisyncknowledge: P0, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISyncKnowledge2>,
    {
        (windows_core::Interface::vtable(self).GetLowestUncontainedId)(windows_core::Interface::as_raw(self), pisyncknowledge.param().abi(), pbitemid, pcbitemidsize).ok()
    }
    pub unsafe fn GetInspector(&self, riid: *const windows_core::GUID, ppiinspector: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetInspector)(windows_core::Interface::as_raw(self), riid, ppiinspector).ok()
    }
    pub unsafe fn GetMinimumSupportedVersion(&self, pversion: *mut SYNC_SERIALIZATION_VERSION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMinimumSupportedVersion)(windows_core::Interface::as_raw(self), pversion).ok()
    }
    pub unsafe fn GetStatistics(&self, which: SYNC_STATISTICS, pvalue: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStatistics)(windows_core::Interface::as_raw(self), which, pvalue).ok()
    }
    pub unsafe fn ContainsKnowledgeForItem<P0>(&self, pknowledge: P0, pbitemid: *const u8) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISyncKnowledge>,
    {
        (windows_core::Interface::vtable(self).ContainsKnowledgeForItem)(windows_core::Interface::as_raw(self), pknowledge.param().abi(), pbitemid).ok()
    }
    pub unsafe fn ContainsKnowledgeForChangeUnit<P0>(&self, pknowledge: P0, pbitemid: *const u8, pbchangeunitid: *const u8) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISyncKnowledge>,
    {
        (windows_core::Interface::vtable(self).ContainsKnowledgeForChangeUnit)(windows_core::Interface::as_raw(self), pknowledge.param().abi(), pbitemid, pbchangeunitid).ok()
    }
    pub unsafe fn ProjectOntoKnowledgeWithPrerequisite<P0, P1>(&self, pprerequisiteknowledge: P0, ptemplateknowledge: P1) -> windows_core::Result<ISyncKnowledge>
    where
        P0: windows_core::Param<ISyncKnowledge>,
        P1: windows_core::Param<ISyncKnowledge>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProjectOntoKnowledgeWithPrerequisite)(windows_core::Interface::as_raw(self), pprerequisiteknowledge.param().abi(), ptemplateknowledge.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Complement<P0>(&self, psyncknowledge: P0) -> windows_core::Result<ISyncKnowledge>
    where
        P0: windows_core::Param<ISyncKnowledge>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Complement)(windows_core::Interface::as_raw(self), psyncknowledge.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IntersectsWithKnowledge<P0>(&self, psyncknowledge: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISyncKnowledge>,
    {
        (windows_core::Interface::vtable(self).IntersectsWithKnowledge)(windows_core::Interface::as_raw(self), psyncknowledge.param().abi()).ok()
    }
    pub unsafe fn GetKnowledgeCookie(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetKnowledgeCookie)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CompareToKnowledgeCookie<P0>(&self, pknowledgecookie: P0, presult: *mut KNOWLEDGE_COOKIE_COMPARISON_RESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).CompareToKnowledgeCookie)(windows_core::Interface::as_raw(self), pknowledgecookie.param().abi(), presult).ok()
    }
}
#[repr(C)]
pub struct ISyncKnowledge2_Vtbl {
    pub base__: ISyncKnowledge_Vtbl,
    pub GetIdParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ID_PARAMETERS) -> windows_core::HRESULT,
    pub ProjectOntoColumnSet: unsafe extern "system" fn(*mut core::ffi::c_void, *const *const u8, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SerializeWithOptions: unsafe extern "system" fn(*mut core::ffi::c_void, SYNC_SERIALIZATION_VERSION, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetLowestUncontainedId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetInspector: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMinimumSupportedVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SYNC_SERIALIZATION_VERSION) -> windows_core::HRESULT,
    pub GetStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, SYNC_STATISTICS, *mut u32) -> windows_core::HRESULT,
    pub ContainsKnowledgeForItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const u8) -> windows_core::HRESULT,
    pub ContainsKnowledgeForChangeUnit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const u8, *const u8) -> windows_core::HRESULT,
    pub ProjectOntoKnowledgeWithPrerequisite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Complement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IntersectsWithKnowledge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetKnowledgeCookie: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CompareToKnowledgeCookie: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut KNOWLEDGE_COOKIE_COMPARISON_RESULT) -> windows_core::HRESULT,
}
pub trait ISyncKnowledge2_Impl: Sized + ISyncKnowledge_Impl {
    fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> windows_core::Result<()>;
    fn ProjectOntoColumnSet(&self, ppcolumns: *const *const u8, count: u32) -> windows_core::Result<ISyncKnowledge2>;
    fn SerializeWithOptions(&self, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> windows_core::Result<()>;
    fn GetLowestUncontainedId(&self, pisyncknowledge: Option<&ISyncKnowledge2>, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> windows_core::Result<()>;
    fn GetInspector(&self, riid: *const windows_core::GUID, ppiinspector: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetMinimumSupportedVersion(&self, pversion: *mut SYNC_SERIALIZATION_VERSION) -> windows_core::Result<()>;
    fn GetStatistics(&self, which: SYNC_STATISTICS, pvalue: *mut u32) -> windows_core::Result<()>;
    fn ContainsKnowledgeForItem(&self, pknowledge: Option<&ISyncKnowledge>, pbitemid: *const u8) -> windows_core::Result<()>;
    fn ContainsKnowledgeForChangeUnit(&self, pknowledge: Option<&ISyncKnowledge>, pbitemid: *const u8, pbchangeunitid: *const u8) -> windows_core::Result<()>;
    fn ProjectOntoKnowledgeWithPrerequisite(&self, pprerequisiteknowledge: Option<&ISyncKnowledge>, ptemplateknowledge: Option<&ISyncKnowledge>) -> windows_core::Result<ISyncKnowledge>;
    fn Complement(&self, psyncknowledge: Option<&ISyncKnowledge>) -> windows_core::Result<ISyncKnowledge>;
    fn IntersectsWithKnowledge(&self, psyncknowledge: Option<&ISyncKnowledge>) -> windows_core::Result<()>;
    fn GetKnowledgeCookie(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn CompareToKnowledgeCookie(&self, pknowledgecookie: Option<&windows_core::IUnknown>, presult: *mut KNOWLEDGE_COOKIE_COMPARISON_RESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncKnowledge2 {}
impl ISyncKnowledge2_Vtbl {
    pub const fn new<Identity: ISyncKnowledge2_Impl, const OFFSET: isize>() -> ISyncKnowledge2_Vtbl {
        unsafe extern "system" fn GetIdParameters<Identity: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge2_Impl::GetIdParameters(this, core::mem::transmute_copy(&pidparameters)).into()
        }
        unsafe extern "system" fn ProjectOntoColumnSet<Identity: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolumns: *const *const u8, count: u32, ppiknowledgeout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncKnowledge2_Impl::ProjectOntoColumnSet(this, core::mem::transmute_copy(&ppcolumns), core::mem::transmute_copy(&count)) {
                Ok(ok__) => {
                    ppiknowledgeout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SerializeWithOptions<Identity: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge2_Impl::SerializeWithOptions(this, core::mem::transmute_copy(&targetformatversion), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pbbuffer), core::mem::transmute_copy(&pdwserializedsize)).into()
        }
        unsafe extern "system" fn GetLowestUncontainedId<Identity: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisyncknowledge: *mut core::ffi::c_void, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge2_Impl::GetLowestUncontainedId(this, windows_core::from_raw_borrowed(&pisyncknowledge), core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pcbitemidsize)).into()
        }
        unsafe extern "system" fn GetInspector<Identity: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppiinspector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge2_Impl::GetInspector(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppiinspector)).into()
        }
        unsafe extern "system" fn GetMinimumSupportedVersion<Identity: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pversion: *mut SYNC_SERIALIZATION_VERSION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge2_Impl::GetMinimumSupportedVersion(this, core::mem::transmute_copy(&pversion)).into()
        }
        unsafe extern "system" fn GetStatistics<Identity: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, which: SYNC_STATISTICS, pvalue: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge2_Impl::GetStatistics(this, core::mem::transmute_copy(&which), core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn ContainsKnowledgeForItem<Identity: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pknowledge: *mut core::ffi::c_void, pbitemid: *const u8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge2_Impl::ContainsKnowledgeForItem(this, windows_core::from_raw_borrowed(&pknowledge), core::mem::transmute_copy(&pbitemid)).into()
        }
        unsafe extern "system" fn ContainsKnowledgeForChangeUnit<Identity: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pknowledge: *mut core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge2_Impl::ContainsKnowledgeForChangeUnit(this, windows_core::from_raw_borrowed(&pknowledge), core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pbchangeunitid)).into()
        }
        unsafe extern "system" fn ProjectOntoKnowledgeWithPrerequisite<Identity: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprerequisiteknowledge: *mut core::ffi::c_void, ptemplateknowledge: *mut core::ffi::c_void, ppprojectedknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncKnowledge2_Impl::ProjectOntoKnowledgeWithPrerequisite(this, windows_core::from_raw_borrowed(&pprerequisiteknowledge), windows_core::from_raw_borrowed(&ptemplateknowledge)) {
                Ok(ok__) => {
                    ppprojectedknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Complement<Identity: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psyncknowledge: *mut core::ffi::c_void, ppcomplementedknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncKnowledge2_Impl::Complement(this, windows_core::from_raw_borrowed(&psyncknowledge)) {
                Ok(ok__) => {
                    ppcomplementedknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IntersectsWithKnowledge<Identity: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psyncknowledge: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge2_Impl::IntersectsWithKnowledge(this, windows_core::from_raw_borrowed(&psyncknowledge)).into()
        }
        unsafe extern "system" fn GetKnowledgeCookie<Identity: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppknowledgecookie: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncKnowledge2_Impl::GetKnowledgeCookie(this) {
                Ok(ok__) => {
                    ppknowledgecookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareToKnowledgeCookie<Identity: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pknowledgecookie: *mut core::ffi::c_void, presult: *mut KNOWLEDGE_COOKIE_COMPARISON_RESULT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge2_Impl::CompareToKnowledgeCookie(this, windows_core::from_raw_borrowed(&pknowledgecookie), core::mem::transmute_copy(&presult)).into()
        }
        Self {
            base__: ISyncKnowledge_Vtbl::new::<Identity, OFFSET>(),
            GetIdParameters: GetIdParameters::<Identity, OFFSET>,
            ProjectOntoColumnSet: ProjectOntoColumnSet::<Identity, OFFSET>,
            SerializeWithOptions: SerializeWithOptions::<Identity, OFFSET>,
            GetLowestUncontainedId: GetLowestUncontainedId::<Identity, OFFSET>,
            GetInspector: GetInspector::<Identity, OFFSET>,
            GetMinimumSupportedVersion: GetMinimumSupportedVersion::<Identity, OFFSET>,
            GetStatistics: GetStatistics::<Identity, OFFSET>,
            ContainsKnowledgeForItem: ContainsKnowledgeForItem::<Identity, OFFSET>,
            ContainsKnowledgeForChangeUnit: ContainsKnowledgeForChangeUnit::<Identity, OFFSET>,
            ProjectOntoKnowledgeWithPrerequisite: ProjectOntoKnowledgeWithPrerequisite::<Identity, OFFSET>,
            Complement: Complement::<Identity, OFFSET>,
            IntersectsWithKnowledge: IntersectsWithKnowledge::<Identity, OFFSET>,
            GetKnowledgeCookie: GetKnowledgeCookie::<Identity, OFFSET>,
            CompareToKnowledgeCookie: CompareToKnowledgeCookie::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncKnowledge2 as windows_core::Interface>::IID || iid == &<ISyncKnowledge as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISyncMergeTombstoneChange, ISyncMergeTombstoneChange_Vtbl, 0x6ec62597_0903_484c_ad61_36d6e938f47b);
impl core::ops::Deref for ISyncMergeTombstoneChange {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISyncMergeTombstoneChange, windows_core::IUnknown);
impl ISyncMergeTombstoneChange {
    pub unsafe fn GetWinnerItemId(&self, pbwinneritemid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetWinnerItemId)(windows_core::Interface::as_raw(self), pbwinneritemid, pcbidsize).ok()
    }
}
#[repr(C)]
pub struct ISyncMergeTombstoneChange_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetWinnerItemId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
}
pub trait ISyncMergeTombstoneChange_Impl: Sized + windows_core::IUnknownImpl {
    fn GetWinnerItemId(&self, pbwinneritemid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncMergeTombstoneChange {}
impl ISyncMergeTombstoneChange_Vtbl {
    pub const fn new<Identity: ISyncMergeTombstoneChange_Impl, const OFFSET: isize>() -> ISyncMergeTombstoneChange_Vtbl {
        unsafe extern "system" fn GetWinnerItemId<Identity: ISyncMergeTombstoneChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbwinneritemid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncMergeTombstoneChange_Impl::GetWinnerItemId(this, core::mem::transmute_copy(&pbwinneritemid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetWinnerItemId: GetWinnerItemId::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncMergeTombstoneChange as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISyncProvider, ISyncProvider_Vtbl, 0x8f657056_2bce_4a17_8c68_c7bb7898b56f);
impl core::ops::Deref for ISyncProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISyncProvider, windows_core::IUnknown);
impl ISyncProvider {
    pub unsafe fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetIdParameters)(windows_core::Interface::as_raw(self), pidparameters).ok()
    }
}
#[repr(C)]
pub struct ISyncProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetIdParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ID_PARAMETERS) -> windows_core::HRESULT,
}
pub trait ISyncProvider_Impl: Sized + windows_core::IUnknownImpl {
    fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncProvider {}
impl ISyncProvider_Vtbl {
    pub const fn new<Identity: ISyncProvider_Impl, const OFFSET: isize>() -> ISyncProvider_Vtbl {
        unsafe extern "system" fn GetIdParameters<Identity: ISyncProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncProvider_Impl::GetIdParameters(this, core::mem::transmute_copy(&pidparameters)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetIdParameters: GetIdParameters::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncProvider as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISyncProviderConfigUI, ISyncProviderConfigUI_Vtbl, 0x7b0705f6_cbcd_4071_ab05_3bdc364d4a0c);
impl core::ops::Deref for ISyncProviderConfigUI {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISyncProviderConfigUI, windows_core::IUnknown);
impl ISyncProviderConfigUI {
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Init<P0>(&self, pguidinstanceid: *const windows_core::GUID, pguidcontenttype: *const windows_core::GUID, pconfigurationproperties: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
    {
        (windows_core::Interface::vtable(self).Init)(windows_core::Interface::as_raw(self), pguidinstanceid, pguidcontenttype, pconfigurationproperties.param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetRegisteredProperties(&self) -> windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRegisteredProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn CreateAndRegisterNewSyncProvider<P0, P1>(&self, hwndparent: P0, punkcontext: P1) -> windows_core::Result<ISyncProviderInfo>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateAndRegisterNewSyncProvider)(windows_core::Interface::as_raw(self), hwndparent.param().abi(), punkcontext.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn ModifySyncProvider<P0, P1, P2>(&self, hwndparent: P0, punkcontext: P1, pproviderinfo: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        P1: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<ISyncProviderInfo>,
    {
        (windows_core::Interface::vtable(self).ModifySyncProvider)(windows_core::Interface::as_raw(self), hwndparent.param().abi(), punkcontext.param().abi(), pproviderinfo.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ISyncProviderConfigUI_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Init: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Init: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetRegisteredProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetRegisteredProperties: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub CreateAndRegisterNewSyncProvider: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    CreateAndRegisterNewSyncProvider: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub ModifySyncProvider: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    ModifySyncProvider: usize,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait ISyncProviderConfigUI_Impl: Sized + windows_core::IUnknownImpl {
    fn Init(&self, pguidinstanceid: *const windows_core::GUID, pguidcontenttype: *const windows_core::GUID, pconfigurationproperties: Option<&super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> windows_core::Result<()>;
    fn GetRegisteredProperties(&self) -> windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn CreateAndRegisterNewSyncProvider(&self, hwndparent: super::super::Foundation::HWND, punkcontext: Option<&windows_core::IUnknown>) -> windows_core::Result<ISyncProviderInfo>;
    fn ModifySyncProvider(&self, hwndparent: super::super::Foundation::HWND, punkcontext: Option<&windows_core::IUnknown>, pproviderinfo: Option<&ISyncProviderInfo>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for ISyncProviderConfigUI {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ISyncProviderConfigUI_Vtbl {
    pub const fn new<Identity: ISyncProviderConfigUI_Impl, const OFFSET: isize>() -> ISyncProviderConfigUI_Vtbl {
        unsafe extern "system" fn Init<Identity: ISyncProviderConfigUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidinstanceid: *const windows_core::GUID, pguidcontenttype: *const windows_core::GUID, pconfigurationproperties: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncProviderConfigUI_Impl::Init(this, core::mem::transmute_copy(&pguidinstanceid), core::mem::transmute_copy(&pguidcontenttype), windows_core::from_raw_borrowed(&pconfigurationproperties)).into()
        }
        unsafe extern "system" fn GetRegisteredProperties<Identity: ISyncProviderConfigUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppconfiguiproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncProviderConfigUI_Impl::GetRegisteredProperties(this) {
                Ok(ok__) => {
                    ppconfiguiproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAndRegisterNewSyncProvider<Identity: ISyncProviderConfigUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, punkcontext: *mut core::ffi::c_void, ppproviderinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncProviderConfigUI_Impl::CreateAndRegisterNewSyncProvider(this, core::mem::transmute_copy(&hwndparent), windows_core::from_raw_borrowed(&punkcontext)) {
                Ok(ok__) => {
                    ppproviderinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifySyncProvider<Identity: ISyncProviderConfigUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, punkcontext: *mut core::ffi::c_void, pproviderinfo: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncProviderConfigUI_Impl::ModifySyncProvider(this, core::mem::transmute_copy(&hwndparent), windows_core::from_raw_borrowed(&punkcontext), windows_core::from_raw_borrowed(&pproviderinfo)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, OFFSET>,
            GetRegisteredProperties: GetRegisteredProperties::<Identity, OFFSET>,
            CreateAndRegisterNewSyncProvider: CreateAndRegisterNewSyncProvider::<Identity, OFFSET>,
            ModifySyncProvider: ModifySyncProvider::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncProviderConfigUI as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
windows_core::imp::define_interface!(ISyncProviderConfigUIInfo, ISyncProviderConfigUIInfo_Vtbl, 0x214141ae_33d7_4d8d_8e37_f227e880ce50);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl core::ops::Deref for ISyncProviderConfigUIInfo {
    type Target = super::super::UI::Shell::PropertiesSystem::IPropertyStore;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
windows_core::imp::interface_hierarchy!(ISyncProviderConfigUIInfo, windows_core::IUnknown, super::super::UI::Shell::PropertiesSystem::IPropertyStore);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ISyncProviderConfigUIInfo {
    pub unsafe fn GetSyncProviderConfigUI(&self, dwclscontext: u32) -> windows_core::Result<ISyncProviderConfigUI> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSyncProviderConfigUI)(windows_core::Interface::as_raw(self), dwclscontext, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[repr(C)]
pub struct ISyncProviderConfigUIInfo_Vtbl {
    pub base__: super::super::UI::Shell::PropertiesSystem::IPropertyStore_Vtbl,
    pub GetSyncProviderConfigUI: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ISyncProviderConfigUIInfo_Impl: Sized + super::super::UI::Shell::PropertiesSystem::IPropertyStore_Impl {
    fn GetSyncProviderConfigUI(&self, dwclscontext: u32) -> windows_core::Result<ISyncProviderConfigUI>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl windows_core::RuntimeName for ISyncProviderConfigUIInfo {}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ISyncProviderConfigUIInfo_Vtbl {
    pub const fn new<Identity: ISyncProviderConfigUIInfo_Impl, const OFFSET: isize>() -> ISyncProviderConfigUIInfo_Vtbl {
        unsafe extern "system" fn GetSyncProviderConfigUI<Identity: ISyncProviderConfigUIInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwclscontext: u32, ppsyncproviderconfigui: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncProviderConfigUIInfo_Impl::GetSyncProviderConfigUI(this, core::mem::transmute_copy(&dwclscontext)) {
                Ok(ok__) => {
                    ppsyncproviderconfigui.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::UI::Shell::PropertiesSystem::IPropertyStore_Vtbl::new::<Identity, OFFSET>(),
            GetSyncProviderConfigUI: GetSyncProviderConfigUI::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncProviderConfigUIInfo as windows_core::Interface>::IID || iid == &<super::super::UI::Shell::PropertiesSystem::IPropertyStore as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
windows_core::imp::define_interface!(ISyncProviderInfo, ISyncProviderInfo_Vtbl, 0x1ee135de_88a4_4504_b0d0_f7920d7e5ba6);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl core::ops::Deref for ISyncProviderInfo {
    type Target = super::super::UI::Shell::PropertiesSystem::IPropertyStore;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
windows_core::imp::interface_hierarchy!(ISyncProviderInfo, windows_core::IUnknown, super::super::UI::Shell::PropertiesSystem::IPropertyStore);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ISyncProviderInfo {
    pub unsafe fn GetSyncProvider(&self, dwclscontext: u32) -> windows_core::Result<IRegisteredSyncProvider> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSyncProvider)(windows_core::Interface::as_raw(self), dwclscontext, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[repr(C)]
pub struct ISyncProviderInfo_Vtbl {
    pub base__: super::super::UI::Shell::PropertiesSystem::IPropertyStore_Vtbl,
    pub GetSyncProvider: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ISyncProviderInfo_Impl: Sized + super::super::UI::Shell::PropertiesSystem::IPropertyStore_Impl {
    fn GetSyncProvider(&self, dwclscontext: u32) -> windows_core::Result<IRegisteredSyncProvider>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl windows_core::RuntimeName for ISyncProviderInfo {}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ISyncProviderInfo_Vtbl {
    pub const fn new<Identity: ISyncProviderInfo_Impl, const OFFSET: isize>() -> ISyncProviderInfo_Vtbl {
        unsafe extern "system" fn GetSyncProvider<Identity: ISyncProviderInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwclscontext: u32, ppsyncprovider: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncProviderInfo_Impl::GetSyncProvider(this, core::mem::transmute_copy(&dwclscontext)) {
                Ok(ok__) => {
                    ppsyncprovider.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::UI::Shell::PropertiesSystem::IPropertyStore_Vtbl::new::<Identity, OFFSET>(),
            GetSyncProvider: GetSyncProvider::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncProviderInfo as windows_core::Interface>::IID || iid == &<super::super::UI::Shell::PropertiesSystem::IPropertyStore as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISyncProviderRegistration, ISyncProviderRegistration_Vtbl, 0xcb45953b_7624_47bc_a472_eb8cac6b222e);
impl core::ops::Deref for ISyncProviderRegistration {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISyncProviderRegistration, windows_core::IUnknown);
impl ISyncProviderRegistration {
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn CreateSyncProviderConfigUIRegistrationInstance(&self, pconfiguiconfig: *const SyncProviderConfigUIConfiguration) -> windows_core::Result<ISyncProviderConfigUIInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSyncProviderConfigUIRegistrationInstance)(windows_core::Interface::as_raw(self), pconfiguiconfig, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UnregisterSyncProviderConfigUI(&self, pguidinstanceid: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnregisterSyncProviderConfigUI)(windows_core::Interface::as_raw(self), pguidinstanceid).ok()
    }
    pub unsafe fn EnumerateSyncProviderConfigUIs(&self, pguidcontenttype: Option<*const windows_core::GUID>, dwsupportedarchitecture: u32) -> windows_core::Result<IEnumSyncProviderConfigUIInfos> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateSyncProviderConfigUIs)(windows_core::Interface::as_raw(self), core::mem::transmute(pguidcontenttype.unwrap_or(core::ptr::null())), dwsupportedarchitecture, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn CreateSyncProviderRegistrationInstance(&self, pproviderconfiguration: *const SyncProviderConfiguration) -> windows_core::Result<ISyncProviderInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSyncProviderRegistrationInstance)(windows_core::Interface::as_raw(self), pproviderconfiguration, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UnregisterSyncProvider(&self, pguidinstanceid: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnregisterSyncProvider)(windows_core::Interface::as_raw(self), pguidinstanceid).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetSyncProviderConfigUIInfoforProvider(&self, pguidproviderinstanceid: *const windows_core::GUID) -> windows_core::Result<ISyncProviderConfigUIInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSyncProviderConfigUIInfoforProvider)(windows_core::Interface::as_raw(self), pguidproviderinstanceid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateSyncProviders(&self, pguidcontenttype: Option<*const windows_core::GUID>, dwstateflagstofiltermask: u32, dwstateflagstofilter: u32, refproviderclsid: *const windows_core::GUID, dwsupportedarchitecture: u32) -> windows_core::Result<IEnumSyncProviderInfos> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateSyncProviders)(windows_core::Interface::as_raw(self), core::mem::transmute(pguidcontenttype.unwrap_or(core::ptr::null())), dwstateflagstofiltermask, dwstateflagstofilter, refproviderclsid, dwsupportedarchitecture, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetSyncProviderInfo(&self, pguidinstanceid: *const windows_core::GUID) -> windows_core::Result<ISyncProviderInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSyncProviderInfo)(windows_core::Interface::as_raw(self), pguidinstanceid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSyncProviderFromInstanceId(&self, pguidinstanceid: *const windows_core::GUID, dwclscontext: u32) -> windows_core::Result<IRegisteredSyncProvider> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSyncProviderFromInstanceId)(windows_core::Interface::as_raw(self), pguidinstanceid, dwclscontext, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetSyncProviderConfigUIInfo(&self, pguidinstanceid: *const windows_core::GUID) -> windows_core::Result<ISyncProviderConfigUIInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSyncProviderConfigUIInfo)(windows_core::Interface::as_raw(self), pguidinstanceid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSyncProviderConfigUIFromInstanceId(&self, pguidinstanceid: *const windows_core::GUID, dwclscontext: u32) -> windows_core::Result<ISyncProviderConfigUI> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSyncProviderConfigUIFromInstanceId)(windows_core::Interface::as_raw(self), pguidinstanceid, dwclscontext, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSyncProviderState(&self, pguidinstanceid: *const windows_core::GUID) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSyncProviderState)(windows_core::Interface::as_raw(self), pguidinstanceid, &mut result__).map(|| result__)
    }
    pub unsafe fn SetSyncProviderState(&self, pguidinstanceid: *const windows_core::GUID, dwstateflagsmask: u32, dwstateflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSyncProviderState)(windows_core::Interface::as_raw(self), pguidinstanceid, dwstateflagsmask, dwstateflags).ok()
    }
    pub unsafe fn RegisterForEvent(&self, phevent: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RegisterForEvent)(windows_core::Interface::as_raw(self), phevent).ok()
    }
    pub unsafe fn RevokeEvent<P0>(&self, hevent: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
    {
        (windows_core::Interface::vtable(self).RevokeEvent)(windows_core::Interface::as_raw(self), hevent.param().abi()).ok()
    }
    pub unsafe fn GetChange<P0>(&self, hevent: P0) -> windows_core::Result<ISyncRegistrationChange>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetChange)(windows_core::Interface::as_raw(self), hevent.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISyncProviderRegistration_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub CreateSyncProviderConfigUIRegistrationInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *const SyncProviderConfigUIConfiguration, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    CreateSyncProviderConfigUIRegistrationInstance: usize,
    pub UnregisterSyncProviderConfigUI: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub EnumerateSyncProviderConfigUIs: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub CreateSyncProviderRegistrationInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *const SyncProviderConfiguration, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    CreateSyncProviderRegistrationInstance: usize,
    pub UnregisterSyncProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetSyncProviderConfigUIInfoforProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetSyncProviderConfigUIInfoforProvider: usize,
    pub EnumerateSyncProviders: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, u32, *const windows_core::GUID, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetSyncProviderInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetSyncProviderInfo: usize,
    pub GetSyncProviderFromInstanceId: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetSyncProviderConfigUIInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetSyncProviderConfigUIInfo: usize,
    pub GetSyncProviderConfigUIFromInstanceId: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSyncProviderState: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub SetSyncProviderState: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, u32) -> windows_core::HRESULT,
    pub RegisterForEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub RevokeEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub GetChange: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait ISyncProviderRegistration_Impl: Sized + windows_core::IUnknownImpl {
    fn CreateSyncProviderConfigUIRegistrationInstance(&self, pconfiguiconfig: *const SyncProviderConfigUIConfiguration) -> windows_core::Result<ISyncProviderConfigUIInfo>;
    fn UnregisterSyncProviderConfigUI(&self, pguidinstanceid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn EnumerateSyncProviderConfigUIs(&self, pguidcontenttype: *const windows_core::GUID, dwsupportedarchitecture: u32) -> windows_core::Result<IEnumSyncProviderConfigUIInfos>;
    fn CreateSyncProviderRegistrationInstance(&self, pproviderconfiguration: *const SyncProviderConfiguration) -> windows_core::Result<ISyncProviderInfo>;
    fn UnregisterSyncProvider(&self, pguidinstanceid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetSyncProviderConfigUIInfoforProvider(&self, pguidproviderinstanceid: *const windows_core::GUID) -> windows_core::Result<ISyncProviderConfigUIInfo>;
    fn EnumerateSyncProviders(&self, pguidcontenttype: *const windows_core::GUID, dwstateflagstofiltermask: u32, dwstateflagstofilter: u32, refproviderclsid: *const windows_core::GUID, dwsupportedarchitecture: u32) -> windows_core::Result<IEnumSyncProviderInfos>;
    fn GetSyncProviderInfo(&self, pguidinstanceid: *const windows_core::GUID) -> windows_core::Result<ISyncProviderInfo>;
    fn GetSyncProviderFromInstanceId(&self, pguidinstanceid: *const windows_core::GUID, dwclscontext: u32) -> windows_core::Result<IRegisteredSyncProvider>;
    fn GetSyncProviderConfigUIInfo(&self, pguidinstanceid: *const windows_core::GUID) -> windows_core::Result<ISyncProviderConfigUIInfo>;
    fn GetSyncProviderConfigUIFromInstanceId(&self, pguidinstanceid: *const windows_core::GUID, dwclscontext: u32) -> windows_core::Result<ISyncProviderConfigUI>;
    fn GetSyncProviderState(&self, pguidinstanceid: *const windows_core::GUID) -> windows_core::Result<u32>;
    fn SetSyncProviderState(&self, pguidinstanceid: *const windows_core::GUID, dwstateflagsmask: u32, dwstateflags: u32) -> windows_core::Result<()>;
    fn RegisterForEvent(&self, phevent: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn RevokeEvent(&self, hevent: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn GetChange(&self, hevent: super::super::Foundation::HANDLE) -> windows_core::Result<ISyncRegistrationChange>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for ISyncProviderRegistration {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ISyncProviderRegistration_Vtbl {
    pub const fn new<Identity: ISyncProviderRegistration_Impl, const OFFSET: isize>() -> ISyncProviderRegistration_Vtbl {
        unsafe extern "system" fn CreateSyncProviderConfigUIRegistrationInstance<Identity: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconfiguiconfig: *const SyncProviderConfigUIConfiguration, ppconfiguiinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncProviderRegistration_Impl::CreateSyncProviderConfigUIRegistrationInstance(this, core::mem::transmute_copy(&pconfiguiconfig)) {
                Ok(ok__) => {
                    ppconfiguiinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterSyncProviderConfigUI<Identity: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidinstanceid: *const windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncProviderRegistration_Impl::UnregisterSyncProviderConfigUI(this, core::mem::transmute_copy(&pguidinstanceid)).into()
        }
        unsafe extern "system" fn EnumerateSyncProviderConfigUIs<Identity: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidcontenttype: *const windows_core::GUID, dwsupportedarchitecture: u32, ppenumsyncproviderconfiguiinfos: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncProviderRegistration_Impl::EnumerateSyncProviderConfigUIs(this, core::mem::transmute_copy(&pguidcontenttype), core::mem::transmute_copy(&dwsupportedarchitecture)) {
                Ok(ok__) => {
                    ppenumsyncproviderconfiguiinfos.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSyncProviderRegistrationInstance<Identity: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproviderconfiguration: *const SyncProviderConfiguration, ppproviderinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncProviderRegistration_Impl::CreateSyncProviderRegistrationInstance(this, core::mem::transmute_copy(&pproviderconfiguration)) {
                Ok(ok__) => {
                    ppproviderinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterSyncProvider<Identity: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidinstanceid: *const windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncProviderRegistration_Impl::UnregisterSyncProvider(this, core::mem::transmute_copy(&pguidinstanceid)).into()
        }
        unsafe extern "system" fn GetSyncProviderConfigUIInfoforProvider<Identity: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidproviderinstanceid: *const windows_core::GUID, ppproviderconfiguiinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncProviderRegistration_Impl::GetSyncProviderConfigUIInfoforProvider(this, core::mem::transmute_copy(&pguidproviderinstanceid)) {
                Ok(ok__) => {
                    ppproviderconfiguiinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateSyncProviders<Identity: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidcontenttype: *const windows_core::GUID, dwstateflagstofiltermask: u32, dwstateflagstofilter: u32, refproviderclsid: *const windows_core::GUID, dwsupportedarchitecture: u32, ppenumsyncproviderinfos: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncProviderRegistration_Impl::EnumerateSyncProviders(this, core::mem::transmute_copy(&pguidcontenttype), core::mem::transmute_copy(&dwstateflagstofiltermask), core::mem::transmute_copy(&dwstateflagstofilter), core::mem::transmute_copy(&refproviderclsid), core::mem::transmute_copy(&dwsupportedarchitecture)) {
                Ok(ok__) => {
                    ppenumsyncproviderinfos.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncProviderInfo<Identity: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidinstanceid: *const windows_core::GUID, ppproviderinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncProviderRegistration_Impl::GetSyncProviderInfo(this, core::mem::transmute_copy(&pguidinstanceid)) {
                Ok(ok__) => {
                    ppproviderinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncProviderFromInstanceId<Identity: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidinstanceid: *const windows_core::GUID, dwclscontext: u32, ppsyncprovider: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncProviderRegistration_Impl::GetSyncProviderFromInstanceId(this, core::mem::transmute_copy(&pguidinstanceid), core::mem::transmute_copy(&dwclscontext)) {
                Ok(ok__) => {
                    ppsyncprovider.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncProviderConfigUIInfo<Identity: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidinstanceid: *const windows_core::GUID, ppconfiguiinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncProviderRegistration_Impl::GetSyncProviderConfigUIInfo(this, core::mem::transmute_copy(&pguidinstanceid)) {
                Ok(ok__) => {
                    ppconfiguiinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncProviderConfigUIFromInstanceId<Identity: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidinstanceid: *const windows_core::GUID, dwclscontext: u32, ppconfigui: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncProviderRegistration_Impl::GetSyncProviderConfigUIFromInstanceId(this, core::mem::transmute_copy(&pguidinstanceid), core::mem::transmute_copy(&dwclscontext)) {
                Ok(ok__) => {
                    ppconfigui.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncProviderState<Identity: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidinstanceid: *const windows_core::GUID, pdwstateflags: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncProviderRegistration_Impl::GetSyncProviderState(this, core::mem::transmute_copy(&pguidinstanceid)) {
                Ok(ok__) => {
                    pdwstateflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSyncProviderState<Identity: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidinstanceid: *const windows_core::GUID, dwstateflagsmask: u32, dwstateflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncProviderRegistration_Impl::SetSyncProviderState(this, core::mem::transmute_copy(&pguidinstanceid), core::mem::transmute_copy(&dwstateflagsmask), core::mem::transmute_copy(&dwstateflags)).into()
        }
        unsafe extern "system" fn RegisterForEvent<Identity: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phevent: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncProviderRegistration_Impl::RegisterForEvent(this, core::mem::transmute_copy(&phevent)).into()
        }
        unsafe extern "system" fn RevokeEvent<Identity: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncProviderRegistration_Impl::RevokeEvent(this, core::mem::transmute_copy(&hevent)).into()
        }
        unsafe extern "system" fn GetChange<Identity: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::super::Foundation::HANDLE, ppchange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncProviderRegistration_Impl::GetChange(this, core::mem::transmute_copy(&hevent)) {
                Ok(ok__) => {
                    ppchange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateSyncProviderConfigUIRegistrationInstance: CreateSyncProviderConfigUIRegistrationInstance::<Identity, OFFSET>,
            UnregisterSyncProviderConfigUI: UnregisterSyncProviderConfigUI::<Identity, OFFSET>,
            EnumerateSyncProviderConfigUIs: EnumerateSyncProviderConfigUIs::<Identity, OFFSET>,
            CreateSyncProviderRegistrationInstance: CreateSyncProviderRegistrationInstance::<Identity, OFFSET>,
            UnregisterSyncProvider: UnregisterSyncProvider::<Identity, OFFSET>,
            GetSyncProviderConfigUIInfoforProvider: GetSyncProviderConfigUIInfoforProvider::<Identity, OFFSET>,
            EnumerateSyncProviders: EnumerateSyncProviders::<Identity, OFFSET>,
            GetSyncProviderInfo: GetSyncProviderInfo::<Identity, OFFSET>,
            GetSyncProviderFromInstanceId: GetSyncProviderFromInstanceId::<Identity, OFFSET>,
            GetSyncProviderConfigUIInfo: GetSyncProviderConfigUIInfo::<Identity, OFFSET>,
            GetSyncProviderConfigUIFromInstanceId: GetSyncProviderConfigUIFromInstanceId::<Identity, OFFSET>,
            GetSyncProviderState: GetSyncProviderState::<Identity, OFFSET>,
            SetSyncProviderState: SetSyncProviderState::<Identity, OFFSET>,
            RegisterForEvent: RegisterForEvent::<Identity, OFFSET>,
            RevokeEvent: RevokeEvent::<Identity, OFFSET>,
            GetChange: GetChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncProviderRegistration as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISyncRegistrationChange, ISyncRegistrationChange_Vtbl, 0xeea0d9ae_6b29_43b4_9e70_e3ae33bb2c3b);
impl core::ops::Deref for ISyncRegistrationChange {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISyncRegistrationChange, windows_core::IUnknown);
impl ISyncRegistrationChange {
    pub unsafe fn GetEvent(&self) -> windows_core::Result<SYNC_REGISTRATION_EVENT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEvent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetInstanceId(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInstanceId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ISyncRegistrationChange_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SYNC_REGISTRATION_EVENT) -> windows_core::HRESULT,
    pub GetInstanceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
pub trait ISyncRegistrationChange_Impl: Sized + windows_core::IUnknownImpl {
    fn GetEvent(&self) -> windows_core::Result<SYNC_REGISTRATION_EVENT>;
    fn GetInstanceId(&self) -> windows_core::Result<windows_core::GUID>;
}
impl windows_core::RuntimeName for ISyncRegistrationChange {}
impl ISyncRegistrationChange_Vtbl {
    pub const fn new<Identity: ISyncRegistrationChange_Impl, const OFFSET: isize>() -> ISyncRegistrationChange_Vtbl {
        unsafe extern "system" fn GetEvent<Identity: ISyncRegistrationChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psreevent: *mut SYNC_REGISTRATION_EVENT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncRegistrationChange_Impl::GetEvent(this) {
                Ok(ok__) => {
                    psreevent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstanceId<Identity: ISyncRegistrationChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidinstanceid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncRegistrationChange_Impl::GetInstanceId(this) {
                Ok(ok__) => {
                    pguidinstanceid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetEvent: GetEvent::<Identity, OFFSET>,
            GetInstanceId: GetInstanceId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncRegistrationChange as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISyncSessionExtendedErrorInfo, ISyncSessionExtendedErrorInfo_Vtbl, 0x326c6810_790a_409b_b741_6999388761eb);
impl core::ops::Deref for ISyncSessionExtendedErrorInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISyncSessionExtendedErrorInfo, windows_core::IUnknown);
impl ISyncSessionExtendedErrorInfo {
    pub unsafe fn GetSyncProviderWithError(&self) -> windows_core::Result<ISyncProvider> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSyncProviderWithError)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISyncSessionExtendedErrorInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSyncProviderWithError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISyncSessionExtendedErrorInfo_Impl: Sized + windows_core::IUnknownImpl {
    fn GetSyncProviderWithError(&self) -> windows_core::Result<ISyncProvider>;
}
impl windows_core::RuntimeName for ISyncSessionExtendedErrorInfo {}
impl ISyncSessionExtendedErrorInfo_Vtbl {
    pub const fn new<Identity: ISyncSessionExtendedErrorInfo_Impl, const OFFSET: isize>() -> ISyncSessionExtendedErrorInfo_Vtbl {
        unsafe extern "system" fn GetSyncProviderWithError<Identity: ISyncSessionExtendedErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproviderwitherror: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncSessionExtendedErrorInfo_Impl::GetSyncProviderWithError(this) {
                Ok(ok__) => {
                    ppproviderwitherror.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetSyncProviderWithError: GetSyncProviderWithError::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncSessionExtendedErrorInfo as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISyncSessionState, ISyncSessionState_Vtbl, 0xb8a940fe_9f01_483b_9434_c37d361225d9);
impl core::ops::Deref for ISyncSessionState {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISyncSessionState, windows_core::IUnknown);
impl ISyncSessionState {
    pub unsafe fn IsCanceled(&self, pfiscanceled: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsCanceled)(windows_core::Interface::as_raw(self), pfiscanceled).ok()
    }
    pub unsafe fn GetInfoForChangeApplication(&self, pbchangeapplierinfo: *mut u8, pcbchangeapplierinfo: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetInfoForChangeApplication)(windows_core::Interface::as_raw(self), pbchangeapplierinfo, pcbchangeapplierinfo).ok()
    }
    pub unsafe fn LoadInfoFromChangeApplication(&self, pbchangeapplierinfo: *const u8, cbchangeapplierinfo: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LoadInfoFromChangeApplication)(windows_core::Interface::as_raw(self), pbchangeapplierinfo, cbchangeapplierinfo).ok()
    }
    pub unsafe fn GetForgottenKnowledgeRecoveryRangeStart(&self, pbrangestart: *mut u8, pcbrangestart: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetForgottenKnowledgeRecoveryRangeStart)(windows_core::Interface::as_raw(self), pbrangestart, pcbrangestart).ok()
    }
    pub unsafe fn GetForgottenKnowledgeRecoveryRangeEnd(&self, pbrangeend: *mut u8, pcbrangeend: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetForgottenKnowledgeRecoveryRangeEnd)(windows_core::Interface::as_raw(self), pbrangeend, pcbrangeend).ok()
    }
    pub unsafe fn SetForgottenKnowledgeRecoveryRange(&self, prange: *const SYNC_RANGE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetForgottenKnowledgeRecoveryRange)(windows_core::Interface::as_raw(self), prange).ok()
    }
    pub unsafe fn OnProgress(&self, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnProgress)(windows_core::Interface::as_raw(self), provider, syncstage, dwcompletedwork, dwtotalwork).ok()
    }
}
#[repr(C)]
pub struct ISyncSessionState_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsCanceled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetInfoForChangeApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub LoadInfoFromChangeApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub GetForgottenKnowledgeRecoveryRangeStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetForgottenKnowledgeRecoveryRangeEnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub SetForgottenKnowledgeRecoveryRange: unsafe extern "system" fn(*mut core::ffi::c_void, *const SYNC_RANGE) -> windows_core::HRESULT,
    pub OnProgress: unsafe extern "system" fn(*mut core::ffi::c_void, SYNC_PROVIDER_ROLE, SYNC_PROGRESS_STAGE, u32, u32) -> windows_core::HRESULT,
}
pub trait ISyncSessionState_Impl: Sized + windows_core::IUnknownImpl {
    fn IsCanceled(&self, pfiscanceled: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetInfoForChangeApplication(&self, pbchangeapplierinfo: *mut u8, pcbchangeapplierinfo: *mut u32) -> windows_core::Result<()>;
    fn LoadInfoFromChangeApplication(&self, pbchangeapplierinfo: *const u8, cbchangeapplierinfo: u32) -> windows_core::Result<()>;
    fn GetForgottenKnowledgeRecoveryRangeStart(&self, pbrangestart: *mut u8, pcbrangestart: *mut u32) -> windows_core::Result<()>;
    fn GetForgottenKnowledgeRecoveryRangeEnd(&self, pbrangeend: *mut u8, pcbrangeend: *mut u32) -> windows_core::Result<()>;
    fn SetForgottenKnowledgeRecoveryRange(&self, prange: *const SYNC_RANGE) -> windows_core::Result<()>;
    fn OnProgress(&self, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncSessionState {}
impl ISyncSessionState_Vtbl {
    pub const fn new<Identity: ISyncSessionState_Impl, const OFFSET: isize>() -> ISyncSessionState_Vtbl {
        unsafe extern "system" fn IsCanceled<Identity: ISyncSessionState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfiscanceled: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncSessionState_Impl::IsCanceled(this, core::mem::transmute_copy(&pfiscanceled)).into()
        }
        unsafe extern "system" fn GetInfoForChangeApplication<Identity: ISyncSessionState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbchangeapplierinfo: *mut u8, pcbchangeapplierinfo: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncSessionState_Impl::GetInfoForChangeApplication(this, core::mem::transmute_copy(&pbchangeapplierinfo), core::mem::transmute_copy(&pcbchangeapplierinfo)).into()
        }
        unsafe extern "system" fn LoadInfoFromChangeApplication<Identity: ISyncSessionState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbchangeapplierinfo: *const u8, cbchangeapplierinfo: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncSessionState_Impl::LoadInfoFromChangeApplication(this, core::mem::transmute_copy(&pbchangeapplierinfo), core::mem::transmute_copy(&cbchangeapplierinfo)).into()
        }
        unsafe extern "system" fn GetForgottenKnowledgeRecoveryRangeStart<Identity: ISyncSessionState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbrangestart: *mut u8, pcbrangestart: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncSessionState_Impl::GetForgottenKnowledgeRecoveryRangeStart(this, core::mem::transmute_copy(&pbrangestart), core::mem::transmute_copy(&pcbrangestart)).into()
        }
        unsafe extern "system" fn GetForgottenKnowledgeRecoveryRangeEnd<Identity: ISyncSessionState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbrangeend: *mut u8, pcbrangeend: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncSessionState_Impl::GetForgottenKnowledgeRecoveryRangeEnd(this, core::mem::transmute_copy(&pbrangeend), core::mem::transmute_copy(&pcbrangeend)).into()
        }
        unsafe extern "system" fn SetForgottenKnowledgeRecoveryRange<Identity: ISyncSessionState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *const SYNC_RANGE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncSessionState_Impl::SetForgottenKnowledgeRecoveryRange(this, core::mem::transmute_copy(&prange)).into()
        }
        unsafe extern "system" fn OnProgress<Identity: ISyncSessionState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncSessionState_Impl::OnProgress(this, core::mem::transmute_copy(&provider), core::mem::transmute_copy(&syncstage), core::mem::transmute_copy(&dwcompletedwork), core::mem::transmute_copy(&dwtotalwork)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsCanceled: IsCanceled::<Identity, OFFSET>,
            GetInfoForChangeApplication: GetInfoForChangeApplication::<Identity, OFFSET>,
            LoadInfoFromChangeApplication: LoadInfoFromChangeApplication::<Identity, OFFSET>,
            GetForgottenKnowledgeRecoveryRangeStart: GetForgottenKnowledgeRecoveryRangeStart::<Identity, OFFSET>,
            GetForgottenKnowledgeRecoveryRangeEnd: GetForgottenKnowledgeRecoveryRangeEnd::<Identity, OFFSET>,
            SetForgottenKnowledgeRecoveryRange: SetForgottenKnowledgeRecoveryRange::<Identity, OFFSET>,
            OnProgress: OnProgress::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncSessionState as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISyncSessionState2, ISyncSessionState2_Vtbl, 0x9e37cfa3_9e38_4c61_9ca3_ffe810b45ca2);
impl core::ops::Deref for ISyncSessionState2 {
    type Target = ISyncSessionState;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISyncSessionState2, windows_core::IUnknown, ISyncSessionState);
impl ISyncSessionState2 {
    pub unsafe fn SetProviderWithError<P0>(&self, fself: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetProviderWithError)(windows_core::Interface::as_raw(self), fself.param().abi()).ok()
    }
    pub unsafe fn GetSessionErrorStatus(&self, phrsessionerror: *mut windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSessionErrorStatus)(windows_core::Interface::as_raw(self), phrsessionerror).ok()
    }
}
#[repr(C)]
pub struct ISyncSessionState2_Vtbl {
    pub base__: ISyncSessionState_Vtbl,
    pub SetProviderWithError: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetSessionErrorStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait ISyncSessionState2_Impl: Sized + ISyncSessionState_Impl {
    fn SetProviderWithError(&self, fself: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetSessionErrorStatus(&self, phrsessionerror: *mut windows_core::HRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncSessionState2 {}
impl ISyncSessionState2_Vtbl {
    pub const fn new<Identity: ISyncSessionState2_Impl, const OFFSET: isize>() -> ISyncSessionState2_Vtbl {
        unsafe extern "system" fn SetProviderWithError<Identity: ISyncSessionState2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fself: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncSessionState2_Impl::SetProviderWithError(this, core::mem::transmute_copy(&fself)).into()
        }
        unsafe extern "system" fn GetSessionErrorStatus<Identity: ISyncSessionState2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrsessionerror: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncSessionState2_Impl::GetSessionErrorStatus(this, core::mem::transmute_copy(&phrsessionerror)).into()
        }
        Self {
            base__: ISyncSessionState_Vtbl::new::<Identity, OFFSET>(),
            SetProviderWithError: SetProviderWithError::<Identity, OFFSET>,
            GetSessionErrorStatus: GetSessionErrorStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncSessionState2 as windows_core::Interface>::IID || iid == &<ISyncSessionState as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(ISynchronousDataRetriever, ISynchronousDataRetriever_Vtbl, 0x9b22f2a9_a4cd_4648_9d8e_3a510d4da04b);
impl core::ops::Deref for ISynchronousDataRetriever {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISynchronousDataRetriever, windows_core::IUnknown);
impl ISynchronousDataRetriever {
    pub unsafe fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetIdParameters)(windows_core::Interface::as_raw(self), pidparameters).ok()
    }
    pub unsafe fn LoadChangeData<P0>(&self, ploadchangecontext: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<ILoadChangeContext>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LoadChangeData)(windows_core::Interface::as_raw(self), ploadchangecontext.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISynchronousDataRetriever_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetIdParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ID_PARAMETERS) -> windows_core::HRESULT,
    pub LoadChangeData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISynchronousDataRetriever_Impl: Sized + windows_core::IUnknownImpl {
    fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> windows_core::Result<()>;
    fn LoadChangeData(&self, ploadchangecontext: Option<&ILoadChangeContext>) -> windows_core::Result<windows_core::IUnknown>;
}
impl windows_core::RuntimeName for ISynchronousDataRetriever {}
impl ISynchronousDataRetriever_Vtbl {
    pub const fn new<Identity: ISynchronousDataRetriever_Impl, const OFFSET: isize>() -> ISynchronousDataRetriever_Vtbl {
        unsafe extern "system" fn GetIdParameters<Identity: ISynchronousDataRetriever_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISynchronousDataRetriever_Impl::GetIdParameters(this, core::mem::transmute_copy(&pidparameters)).into()
        }
        unsafe extern "system" fn LoadChangeData<Identity: ISynchronousDataRetriever_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ploadchangecontext: *mut core::ffi::c_void, ppunkdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISynchronousDataRetriever_Impl::LoadChangeData(this, windows_core::from_raw_borrowed(&ploadchangecontext)) {
                Ok(ok__) => {
                    ppunkdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIdParameters: GetIdParameters::<Identity, OFFSET>,
            LoadChangeData: LoadChangeData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISynchronousDataRetriever as windows_core::Interface>::IID
    }
}
pub const CCR_COLLISION: CONSTRAINT_CONFLICT_REASON = CONSTRAINT_CONFLICT_REASON(1i32);
pub const CCR_IDENTITY: CONSTRAINT_CONFLICT_REASON = CONSTRAINT_CONFLICT_REASON(3i32);
pub const CCR_NOPARENT: CONSTRAINT_CONFLICT_REASON = CONSTRAINT_CONFLICT_REASON(2i32);
pub const CCR_OTHER: CONSTRAINT_CONFLICT_REASON = CONSTRAINT_CONFLICT_REASON(0i32);
pub const CRP_DESTINATION_PROVIDER_WINS: CONFLICT_RESOLUTION_POLICY = CONFLICT_RESOLUTION_POLICY(1i32);
pub const CRP_LAST: CONFLICT_RESOLUTION_POLICY = CONFLICT_RESOLUTION_POLICY(3i32);
pub const CRP_NONE: CONFLICT_RESOLUTION_POLICY = CONFLICT_RESOLUTION_POLICY(0i32);
pub const CRP_SOURCE_PROVIDER_WINS: CONFLICT_RESOLUTION_POLICY = CONFLICT_RESOLUTION_POLICY(2i32);
pub const FCT_INTERSECTION: FILTER_COMBINATION_TYPE = FILTER_COMBINATION_TYPE(0i32);
pub const FT_CURRENT_ITEMS_AND_VERSIONS_FOR_MOVED_OUT_ITEMS: FILTERING_TYPE = FILTERING_TYPE(1i32);
pub const FT_CURRENT_ITEMS_ONLY: FILTERING_TYPE = FILTERING_TYPE(0i32);
pub const KCCR_COOKIE_KNOWLEDGE_CONTAINED: KNOWLEDGE_COOKIE_COMPARISON_RESULT = KNOWLEDGE_COOKIE_COMPARISON_RESULT(1i32);
pub const KCCR_COOKIE_KNOWLEDGE_CONTAINS: KNOWLEDGE_COOKIE_COMPARISON_RESULT = KNOWLEDGE_COOKIE_COMPARISON_RESULT(2i32);
pub const KCCR_COOKIE_KNOWLEDGE_EQUAL: KNOWLEDGE_COOKIE_COMPARISON_RESULT = KNOWLEDGE_COOKIE_COMPARISON_RESULT(0i32);
pub const KCCR_COOKIE_KNOWLEDGE_NOT_COMPARABLE: KNOWLEDGE_COOKIE_COMPARISON_RESULT = KNOWLEDGE_COOKIE_COMPARISON_RESULT(3i32);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_CAPABILITIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 5 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_CLSID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 3 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_CONTENTTYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 4 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 9 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_ICON: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 11 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_INSTANCEID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 2 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_IS_GLOBAL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 7 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_MENUITEM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 13 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_MENUITEM_NOUI: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 12 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 8 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_SUPPORTED_ARCHITECTURE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 6 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_TOOLTIPS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 10 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_CAPABILITIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 6 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_CLSID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 3 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_CONFIGUI: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 4 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_CONTENTTYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 5 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 9 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_ICON: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 11 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_INSTANCEID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 2 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 8 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_SUPPORTED_ARCHITECTURE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 7 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_TOOLTIPS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 10 };
pub const SCC_CAN_CREATE_WITHOUT_UI: u32 = 1u32;
pub const SCC_CAN_MODIFY_WITHOUT_UI: u32 = 2u32;
pub const SCC_CREATE_NOT_SUPPORTED: u32 = 4u32;
pub const SCC_DEFAULT: u32 = 0u32;
pub const SCC_MODIFY_NOT_SUPPORTED: u32 = 8u32;
pub const SCRA_ACCEPT_DESTINATION_PROVIDER: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(1i32);
pub const SCRA_ACCEPT_SOURCE_PROVIDER: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(2i32);
pub const SCRA_DEFER: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(0i32);
pub const SCRA_MERGE: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(4i32);
pub const SCRA_RENAME_DESTINATION: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(6i32);
pub const SCRA_RENAME_SOURCE: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(5i32);
pub const SCRA_TRANSFER_AND_DEFER: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(3i32);
pub const SFEA_ABORT: SYNC_FULL_ENUMERATION_ACTION = SYNC_FULL_ENUMERATION_ACTION(2i32);
pub const SFEA_FULL_ENUMERATION: SYNC_FULL_ENUMERATION_ACTION = SYNC_FULL_ENUMERATION_ACTION(0i32);
pub const SFEA_PARTIAL_SYNC: SYNC_FULL_ENUMERATION_ACTION = SYNC_FULL_ENUMERATION_ACTION(1i32);
pub const SPC_DEFAULT: u32 = 0u32;
pub const SPR_DESTINATION: SYNC_PROVIDER_ROLE = SYNC_PROVIDER_ROLE(1i32);
pub const SPR_SOURCE: SYNC_PROVIDER_ROLE = SYNC_PROVIDER_ROLE(0i32);
pub const SPS_CHANGE_APPLICATION: SYNC_PROGRESS_STAGE = SYNC_PROGRESS_STAGE(2i32);
pub const SPS_CHANGE_DETECTION: SYNC_PROGRESS_STAGE = SYNC_PROGRESS_STAGE(0i32);
pub const SPS_CHANGE_ENUMERATION: SYNC_PROGRESS_STAGE = SYNC_PROGRESS_STAGE(1i32);
pub const SRA_ACCEPT_DESTINATION_PROVIDER: SYNC_RESOLVE_ACTION = SYNC_RESOLVE_ACTION(1i32);
pub const SRA_ACCEPT_SOURCE_PROVIDER: SYNC_RESOLVE_ACTION = SYNC_RESOLVE_ACTION(2i32);
pub const SRA_DEFER: SYNC_RESOLVE_ACTION = SYNC_RESOLVE_ACTION(0i32);
pub const SRA_LAST: SYNC_RESOLVE_ACTION = SYNC_RESOLVE_ACTION(5i32);
pub const SRA_MERGE: SYNC_RESOLVE_ACTION = SYNC_RESOLVE_ACTION(3i32);
pub const SRA_TRANSFER_AND_DEFER: SYNC_RESOLVE_ACTION = SYNC_RESOLVE_ACTION(4i32);
pub const SRE_CONFIGUI_ADDED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(4i32);
pub const SRE_CONFIGUI_REMOVED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(5i32);
pub const SRE_CONFIGUI_UPDATED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(6i32);
pub const SRE_PROVIDER_ADDED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(0i32);
pub const SRE_PROVIDER_REMOVED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(1i32);
pub const SRE_PROVIDER_STATE_CHANGED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(3i32);
pub const SRE_PROVIDER_UPDATED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(2i32);
pub const SYNC_32_BIT_SUPPORTED: u32 = 1u32;
pub const SYNC_64_BIT_SUPPORTED: u32 = 2u32;
pub const SYNC_CHANGE_FLAG_DELETED: u32 = 1u32;
pub const SYNC_CHANGE_FLAG_DOES_NOT_EXIST: u32 = 2u32;
pub const SYNC_CHANGE_FLAG_GHOST: u32 = 4u32;
pub const SYNC_FILTER_INFO_COMBINED: u32 = 8u32;
pub const SYNC_FILTER_INFO_FLAG_CHANGE_UNIT_LIST: u32 = 2u32;
pub const SYNC_FILTER_INFO_FLAG_CUSTOM: u32 = 4u32;
pub const SYNC_FILTER_INFO_FLAG_ITEM_LIST: u32 = 1u32;
pub const SYNC_PROVIDER_CONFIGUI_CONFIGURATION_VERSION: u32 = 1u32;
pub const SYNC_PROVIDER_CONFIGURATION_VERSION: u32 = 1u32;
pub const SYNC_PROVIDER_STATE_DIRTY: u32 = 2u32;
pub const SYNC_PROVIDER_STATE_ENABLED: u32 = 1u32;
pub const SYNC_SERIALIZATION_VERSION_V1: SYNC_SERIALIZATION_VERSION = SYNC_SERIALIZATION_VERSION(1i32);
pub const SYNC_SERIALIZATION_VERSION_V2: SYNC_SERIALIZATION_VERSION = SYNC_SERIALIZATION_VERSION(4i32);
pub const SYNC_SERIALIZATION_VERSION_V3: SYNC_SERIALIZATION_VERSION = SYNC_SERIALIZATION_VERSION(5i32);
pub const SYNC_SERIALIZE_REPLICA_KEY_MAP: u32 = 1u32;
pub const SYNC_STATISTICS_RANGE_COUNT: SYNC_STATISTICS = SYNC_STATISTICS(0i32);
pub const SYNC_VERSION_FLAG_FROM_FEED: u32 = 1u32;
pub const SYNC_VERSION_FLAG_HAS_BY: u32 = 2u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CONFLICT_RESOLUTION_POLICY(pub i32);
impl windows_core::TypeKind for CONFLICT_RESOLUTION_POLICY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CONFLICT_RESOLUTION_POLICY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CONFLICT_RESOLUTION_POLICY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CONSTRAINT_CONFLICT_REASON(pub i32);
impl windows_core::TypeKind for CONSTRAINT_CONFLICT_REASON {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CONSTRAINT_CONFLICT_REASON {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CONSTRAINT_CONFLICT_REASON").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FILTERING_TYPE(pub i32);
impl windows_core::TypeKind for FILTERING_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FILTERING_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FILTERING_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FILTER_COMBINATION_TYPE(pub i32);
impl windows_core::TypeKind for FILTER_COMBINATION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FILTER_COMBINATION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FILTER_COMBINATION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct KNOWLEDGE_COOKIE_COMPARISON_RESULT(pub i32);
impl windows_core::TypeKind for KNOWLEDGE_COOKIE_COMPARISON_RESULT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for KNOWLEDGE_COOKIE_COMPARISON_RESULT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("KNOWLEDGE_COOKIE_COMPARISON_RESULT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SYNC_CONSTRAINT_RESOLVE_ACTION(pub i32);
impl windows_core::TypeKind for SYNC_CONSTRAINT_RESOLVE_ACTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SYNC_CONSTRAINT_RESOLVE_ACTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SYNC_CONSTRAINT_RESOLVE_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SYNC_FULL_ENUMERATION_ACTION(pub i32);
impl windows_core::TypeKind for SYNC_FULL_ENUMERATION_ACTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SYNC_FULL_ENUMERATION_ACTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SYNC_FULL_ENUMERATION_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SYNC_PROGRESS_STAGE(pub i32);
impl windows_core::TypeKind for SYNC_PROGRESS_STAGE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SYNC_PROGRESS_STAGE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SYNC_PROGRESS_STAGE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SYNC_PROVIDER_ROLE(pub i32);
impl windows_core::TypeKind for SYNC_PROVIDER_ROLE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SYNC_PROVIDER_ROLE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SYNC_PROVIDER_ROLE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SYNC_REGISTRATION_EVENT(pub i32);
impl windows_core::TypeKind for SYNC_REGISTRATION_EVENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SYNC_REGISTRATION_EVENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SYNC_REGISTRATION_EVENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SYNC_RESOLVE_ACTION(pub i32);
impl windows_core::TypeKind for SYNC_RESOLVE_ACTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SYNC_RESOLVE_ACTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SYNC_RESOLVE_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SYNC_SERIALIZATION_VERSION(pub i32);
impl windows_core::TypeKind for SYNC_SERIALIZATION_VERSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SYNC_SERIALIZATION_VERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SYNC_SERIALIZATION_VERSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SYNC_STATISTICS(pub i32);
impl windows_core::TypeKind for SYNC_STATISTICS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SYNC_STATISTICS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SYNC_STATISTICS").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ID_PARAMETERS {
    pub dwSize: u32,
    pub replicaId: ID_PARAMETER_PAIR,
    pub itemId: ID_PARAMETER_PAIR,
    pub changeUnitId: ID_PARAMETER_PAIR,
}
impl windows_core::TypeKind for ID_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for ID_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ID_PARAMETER_PAIR {
    pub fIsVariable: super::super::Foundation::BOOL,
    pub cbIdSize: u16,
}
impl windows_core::TypeKind for ID_PARAMETER_PAIR {
    type TypeKind = windows_core::CopyType;
}
impl Default for ID_PARAMETER_PAIR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SYNC_FILTER_CHANGE {
    pub fMoveIn: super::super::Foundation::BOOL,
    pub moveVersion: SYNC_VERSION,
}
impl windows_core::TypeKind for SYNC_FILTER_CHANGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for SYNC_FILTER_CHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SYNC_RANGE {
    pub pbClosedLowerBound: *mut u8,
    pub pbClosedUpperBound: *mut u8,
}
impl windows_core::TypeKind for SYNC_RANGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for SYNC_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SYNC_SESSION_STATISTICS {
    pub dwChangesApplied: u32,
    pub dwChangesFailed: u32,
}
impl windows_core::TypeKind for SYNC_SESSION_STATISTICS {
    type TypeKind = windows_core::CopyType;
}
impl Default for SYNC_SESSION_STATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SYNC_TIME {
    pub dwDate: u32,
    pub dwTime: u32,
}
impl windows_core::TypeKind for SYNC_TIME {
    type TypeKind = windows_core::CopyType;
}
impl Default for SYNC_TIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SYNC_VERSION {
    pub dwLastUpdatingReplicaKey: u32,
    pub ullTickCount: u64,
}
impl windows_core::TypeKind for SYNC_VERSION {
    type TypeKind = windows_core::CopyType;
}
impl Default for SYNC_VERSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SyncProviderConfigUIConfiguration {
    pub dwVersion: u32,
    pub guidInstanceId: windows_core::GUID,
    pub clsidConfigUI: windows_core::GUID,
    pub guidContentType: windows_core::GUID,
    pub dwCapabilities: u32,
    pub dwSupportedArchitecture: u32,
    pub fIsGlobal: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for SyncProviderConfigUIConfiguration {
    type TypeKind = windows_core::CopyType;
}
impl Default for SyncProviderConfigUIConfiguration {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SyncProviderConfiguration {
    pub dwVersion: u32,
    pub guidInstanceId: windows_core::GUID,
    pub clsidProvider: windows_core::GUID,
    pub guidConfigUIInstanceId: windows_core::GUID,
    pub guidContentType: windows_core::GUID,
    pub dwCapabilities: u32,
    pub dwSupportedArchitecture: u32,
}
impl windows_core::TypeKind for SyncProviderConfiguration {
    type TypeKind = windows_core::CopyType;
}
impl Default for SyncProviderConfiguration {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SyncProviderRegistration: windows_core::GUID = windows_core::GUID::from_u128(0xf82b4ef1_93a9_4dde_8015_f7950a1a6e31);
