windows_core::imp::define_interface!(IUIAnimationInterpolator, IUIAnimationInterpolator_Vtbl, 0x7815cbba_ddf7_478c_a46c_7b6c738b7978);
impl core::ops::Deref for IUIAnimationInterpolator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAnimationInterpolator, windows_core::IUnknown);
impl IUIAnimationInterpolator {
    pub unsafe fn SetInitialValueAndVelocity(&self, initialvalue: f64, initialvelocity: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetInitialValueAndVelocity)(windows_core::Interface::as_raw(self), initialvalue, initialvelocity).ok()
    }
    pub unsafe fn SetDuration(&self, duration: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDuration)(windows_core::Interface::as_raw(self), duration).ok()
    }
    pub unsafe fn GetDuration(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDuration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetFinalValue(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFinalValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn InterpolateValue(&self, offset: f64) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InterpolateValue)(windows_core::Interface::as_raw(self), offset, &mut result__).map(|| result__)
    }
    pub unsafe fn InterpolateVelocity(&self, offset: f64) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InterpolateVelocity)(windows_core::Interface::as_raw(self), offset, &mut result__).map(|| result__)
    }
    pub unsafe fn GetDependencies(&self, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDependencies)(windows_core::Interface::as_raw(self), initialvaluedependencies, initialvelocitydependencies, durationdependencies).ok()
    }
}
#[repr(C)]
pub struct IUIAnimationInterpolator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetInitialValueAndVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub GetFinalValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub InterpolateValue: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *mut f64) -> windows_core::HRESULT,
    pub InterpolateVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *mut f64) -> windows_core::HRESULT,
    pub GetDependencies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UI_ANIMATION_DEPENDENCIES, *mut UI_ANIMATION_DEPENDENCIES, *mut UI_ANIMATION_DEPENDENCIES) -> windows_core::HRESULT,
}
pub trait IUIAnimationInterpolator_Impl: Sized + windows_core::IUnknownImpl {
    fn SetInitialValueAndVelocity(&self, initialvalue: f64, initialvelocity: f64) -> windows_core::Result<()>;
    fn SetDuration(&self, duration: f64) -> windows_core::Result<()>;
    fn GetDuration(&self) -> windows_core::Result<f64>;
    fn GetFinalValue(&self) -> windows_core::Result<f64>;
    fn InterpolateValue(&self, offset: f64) -> windows_core::Result<f64>;
    fn InterpolateVelocity(&self, offset: f64) -> windows_core::Result<f64>;
    fn GetDependencies(&self, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationInterpolator {}
impl IUIAnimationInterpolator_Vtbl {
    pub const fn new<Identity: IUIAnimationInterpolator_Impl, const OFFSET: isize>() -> IUIAnimationInterpolator_Vtbl {
        unsafe extern "system" fn SetInitialValueAndVelocity<Identity: IUIAnimationInterpolator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, initialvalue: f64, initialvelocity: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationInterpolator_Impl::SetInitialValueAndVelocity(this, core::mem::transmute_copy(&initialvalue), core::mem::transmute_copy(&initialvelocity)).into()
        }
        unsafe extern "system" fn SetDuration<Identity: IUIAnimationInterpolator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationInterpolator_Impl::SetDuration(this, core::mem::transmute_copy(&duration)).into()
        }
        unsafe extern "system" fn GetDuration<Identity: IUIAnimationInterpolator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationInterpolator_Impl::GetDuration(this) {
                Ok(ok__) => {
                    duration.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalValue<Identity: IUIAnimationInterpolator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationInterpolator_Impl::GetFinalValue(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterpolateValue<Identity: IUIAnimationInterpolator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: f64, value: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationInterpolator_Impl::InterpolateValue(this, core::mem::transmute_copy(&offset)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterpolateVelocity<Identity: IUIAnimationInterpolator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: f64, velocity: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationInterpolator_Impl::InterpolateVelocity(this, core::mem::transmute_copy(&offset)) {
                Ok(ok__) => {
                    velocity.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDependencies<Identity: IUIAnimationInterpolator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationInterpolator_Impl::GetDependencies(this, core::mem::transmute_copy(&initialvaluedependencies), core::mem::transmute_copy(&initialvelocitydependencies), core::mem::transmute_copy(&durationdependencies)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetInitialValueAndVelocity: SetInitialValueAndVelocity::<Identity, OFFSET>,
            SetDuration: SetDuration::<Identity, OFFSET>,
            GetDuration: GetDuration::<Identity, OFFSET>,
            GetFinalValue: GetFinalValue::<Identity, OFFSET>,
            InterpolateValue: InterpolateValue::<Identity, OFFSET>,
            InterpolateVelocity: InterpolateVelocity::<Identity, OFFSET>,
            GetDependencies: GetDependencies::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationInterpolator as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIAnimationInterpolator2, IUIAnimationInterpolator2_Vtbl, 0xea76aff8_ea22_4a23_a0ef_a6a966703518);
impl core::ops::Deref for IUIAnimationInterpolator2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAnimationInterpolator2, windows_core::IUnknown);
impl IUIAnimationInterpolator2 {
    pub unsafe fn GetDimension(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDimension)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetInitialValueAndVelocity(&self, initialvalue: *const f64, initialvelocity: *const f64, cdimension: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetInitialValueAndVelocity)(windows_core::Interface::as_raw(self), initialvalue, initialvelocity, cdimension).ok()
    }
    pub unsafe fn SetDuration(&self, duration: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDuration)(windows_core::Interface::as_raw(self), duration).ok()
    }
    pub unsafe fn GetDuration(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDuration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetFinalValue(&self, value: &mut [f64]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFinalValue)(windows_core::Interface::as_raw(self), core::mem::transmute(value.as_ptr()), value.len().try_into().unwrap()).ok()
    }
    pub unsafe fn InterpolateValue(&self, offset: f64, value: &mut [f64]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InterpolateValue)(windows_core::Interface::as_raw(self), offset, core::mem::transmute(value.as_ptr()), value.len().try_into().unwrap()).ok()
    }
    pub unsafe fn InterpolateVelocity(&self, offset: f64, velocity: &mut [f64]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InterpolateVelocity)(windows_core::Interface::as_raw(self), offset, core::mem::transmute(velocity.as_ptr()), velocity.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetPrimitiveInterpolation<P0>(&self, interpolation: P0, cdimension: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationPrimitiveInterpolation>,
    {
        (windows_core::Interface::vtable(self).GetPrimitiveInterpolation)(windows_core::Interface::as_raw(self), interpolation.param().abi(), cdimension).ok()
    }
    pub unsafe fn GetDependencies(&self, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDependencies)(windows_core::Interface::as_raw(self), initialvaluedependencies, initialvelocitydependencies, durationdependencies).ok()
    }
}
#[repr(C)]
pub struct IUIAnimationInterpolator2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDimension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetInitialValueAndVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, *const f64, *const f64, u32) -> windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub GetFinalValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64, u32) -> windows_core::HRESULT,
    pub InterpolateValue: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *mut f64, u32) -> windows_core::HRESULT,
    pub InterpolateVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *mut f64, u32) -> windows_core::HRESULT,
    pub GetPrimitiveInterpolation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetDependencies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UI_ANIMATION_DEPENDENCIES, *mut UI_ANIMATION_DEPENDENCIES, *mut UI_ANIMATION_DEPENDENCIES) -> windows_core::HRESULT,
}
pub trait IUIAnimationInterpolator2_Impl: Sized + windows_core::IUnknownImpl {
    fn GetDimension(&self) -> windows_core::Result<u32>;
    fn SetInitialValueAndVelocity(&self, initialvalue: *const f64, initialvelocity: *const f64, cdimension: u32) -> windows_core::Result<()>;
    fn SetDuration(&self, duration: f64) -> windows_core::Result<()>;
    fn GetDuration(&self) -> windows_core::Result<f64>;
    fn GetFinalValue(&self, value: *mut f64, cdimension: u32) -> windows_core::Result<()>;
    fn InterpolateValue(&self, offset: f64, value: *mut f64, cdimension: u32) -> windows_core::Result<()>;
    fn InterpolateVelocity(&self, offset: f64, velocity: *mut f64, cdimension: u32) -> windows_core::Result<()>;
    fn GetPrimitiveInterpolation(&self, interpolation: Option<&IUIAnimationPrimitiveInterpolation>, cdimension: u32) -> windows_core::Result<()>;
    fn GetDependencies(&self, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationInterpolator2 {}
impl IUIAnimationInterpolator2_Vtbl {
    pub const fn new<Identity: IUIAnimationInterpolator2_Impl, const OFFSET: isize>() -> IUIAnimationInterpolator2_Vtbl {
        unsafe extern "system" fn GetDimension<Identity: IUIAnimationInterpolator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dimension: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationInterpolator2_Impl::GetDimension(this) {
                Ok(ok__) => {
                    dimension.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialValueAndVelocity<Identity: IUIAnimationInterpolator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, initialvalue: *const f64, initialvelocity: *const f64, cdimension: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationInterpolator2_Impl::SetInitialValueAndVelocity(this, core::mem::transmute_copy(&initialvalue), core::mem::transmute_copy(&initialvelocity), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn SetDuration<Identity: IUIAnimationInterpolator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationInterpolator2_Impl::SetDuration(this, core::mem::transmute_copy(&duration)).into()
        }
        unsafe extern "system" fn GetDuration<Identity: IUIAnimationInterpolator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationInterpolator2_Impl::GetDuration(this) {
                Ok(ok__) => {
                    duration.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalValue<Identity: IUIAnimationInterpolator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut f64, cdimension: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationInterpolator2_Impl::GetFinalValue(this, core::mem::transmute_copy(&value), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn InterpolateValue<Identity: IUIAnimationInterpolator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: f64, value: *mut f64, cdimension: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationInterpolator2_Impl::InterpolateValue(this, core::mem::transmute_copy(&offset), core::mem::transmute_copy(&value), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn InterpolateVelocity<Identity: IUIAnimationInterpolator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: f64, velocity: *mut f64, cdimension: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationInterpolator2_Impl::InterpolateVelocity(this, core::mem::transmute_copy(&offset), core::mem::transmute_copy(&velocity), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetPrimitiveInterpolation<Identity: IUIAnimationInterpolator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interpolation: *mut core::ffi::c_void, cdimension: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationInterpolator2_Impl::GetPrimitiveInterpolation(this, windows_core::from_raw_borrowed(&interpolation), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetDependencies<Identity: IUIAnimationInterpolator2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, initialvaluedependencies: *mut UI_ANIMATION_DEPENDENCIES, initialvelocitydependencies: *mut UI_ANIMATION_DEPENDENCIES, durationdependencies: *mut UI_ANIMATION_DEPENDENCIES) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationInterpolator2_Impl::GetDependencies(this, core::mem::transmute_copy(&initialvaluedependencies), core::mem::transmute_copy(&initialvelocitydependencies), core::mem::transmute_copy(&durationdependencies)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDimension: GetDimension::<Identity, OFFSET>,
            SetInitialValueAndVelocity: SetInitialValueAndVelocity::<Identity, OFFSET>,
            SetDuration: SetDuration::<Identity, OFFSET>,
            GetDuration: GetDuration::<Identity, OFFSET>,
            GetFinalValue: GetFinalValue::<Identity, OFFSET>,
            InterpolateValue: InterpolateValue::<Identity, OFFSET>,
            InterpolateVelocity: InterpolateVelocity::<Identity, OFFSET>,
            GetPrimitiveInterpolation: GetPrimitiveInterpolation::<Identity, OFFSET>,
            GetDependencies: GetDependencies::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationInterpolator2 as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIAnimationLoopIterationChangeHandler2, IUIAnimationLoopIterationChangeHandler2_Vtbl, 0x2d3b15a4_4762_47ab_a030_b23221df3ae0);
impl core::ops::Deref for IUIAnimationLoopIterationChangeHandler2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAnimationLoopIterationChangeHandler2, windows_core::IUnknown);
impl IUIAnimationLoopIterationChangeHandler2 {
    pub unsafe fn OnLoopIterationChanged<P0>(&self, storyboard: P0, id: usize, newiterationcount: u32, olditerationcount: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationStoryboard2>,
    {
        (windows_core::Interface::vtable(self).OnLoopIterationChanged)(windows_core::Interface::as_raw(self), storyboard.param().abi(), id, newiterationcount, olditerationcount).ok()
    }
}
#[repr(C)]
pub struct IUIAnimationLoopIterationChangeHandler2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnLoopIterationChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, usize, u32, u32) -> windows_core::HRESULT,
}
pub trait IUIAnimationLoopIterationChangeHandler2_Impl: Sized + windows_core::IUnknownImpl {
    fn OnLoopIterationChanged(&self, storyboard: Option<&IUIAnimationStoryboard2>, id: usize, newiterationcount: u32, olditerationcount: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationLoopIterationChangeHandler2 {}
impl IUIAnimationLoopIterationChangeHandler2_Vtbl {
    pub const fn new<Identity: IUIAnimationLoopIterationChangeHandler2_Impl, const OFFSET: isize>() -> IUIAnimationLoopIterationChangeHandler2_Vtbl {
        unsafe extern "system" fn OnLoopIterationChanged<Identity: IUIAnimationLoopIterationChangeHandler2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut core::ffi::c_void, id: usize, newiterationcount: u32, olditerationcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationLoopIterationChangeHandler2_Impl::OnLoopIterationChanged(this, windows_core::from_raw_borrowed(&storyboard), core::mem::transmute_copy(&id), core::mem::transmute_copy(&newiterationcount), core::mem::transmute_copy(&olditerationcount)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnLoopIterationChanged: OnLoopIterationChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationLoopIterationChangeHandler2 as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIAnimationManager, IUIAnimationManager_Vtbl, 0x9169896c_ac8d_4e7d_94e5_67fa4dc2f2e8);
impl core::ops::Deref for IUIAnimationManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAnimationManager, windows_core::IUnknown);
impl IUIAnimationManager {
    pub unsafe fn CreateAnimationVariable(&self, initialvalue: f64) -> windows_core::Result<IUIAnimationVariable> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateAnimationVariable)(windows_core::Interface::as_raw(self), initialvalue, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ScheduleTransition<P0, P1>(&self, variable: P0, transition: P1, timenow: f64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationVariable>,
        P1: windows_core::Param<IUIAnimationTransition>,
    {
        (windows_core::Interface::vtable(self).ScheduleTransition)(windows_core::Interface::as_raw(self), variable.param().abi(), transition.param().abi(), timenow).ok()
    }
    pub unsafe fn CreateStoryboard(&self) -> windows_core::Result<IUIAnimationStoryboard> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateStoryboard)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FinishAllStoryboards(&self, completiondeadline: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FinishAllStoryboards)(windows_core::Interface::as_raw(self), completiondeadline).ok()
    }
    pub unsafe fn AbandonAllStoryboards(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AbandonAllStoryboards)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Update(&self, timenow: f64, updateresult: Option<*mut UI_ANIMATION_UPDATE_RESULT>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), timenow, core::mem::transmute(updateresult.unwrap_or(core::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetVariableFromTag<P0>(&self, object: P0, id: u32) -> windows_core::Result<IUIAnimationVariable>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVariableFromTag)(windows_core::Interface::as_raw(self), object.param().abi(), id, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetStoryboardFromTag<P0>(&self, object: P0, id: u32) -> windows_core::Result<IUIAnimationStoryboard>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStoryboardFromTag)(windows_core::Interface::as_raw(self), object.param().abi(), id, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetStatus(&self) -> windows_core::Result<UI_ANIMATION_MANAGER_STATUS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAnimationMode(&self, mode: UI_ANIMATION_MODE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAnimationMode)(windows_core::Interface::as_raw(self), mode).ok()
    }
    pub unsafe fn Pause(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetManagerEventHandler<P0>(&self, handler: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationManagerEventHandler>,
    {
        (windows_core::Interface::vtable(self).SetManagerEventHandler)(windows_core::Interface::as_raw(self), handler.param().abi()).ok()
    }
    pub unsafe fn SetCancelPriorityComparison<P0>(&self, comparison: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationPriorityComparison>,
    {
        (windows_core::Interface::vtable(self).SetCancelPriorityComparison)(windows_core::Interface::as_raw(self), comparison.param().abi()).ok()
    }
    pub unsafe fn SetTrimPriorityComparison<P0>(&self, comparison: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationPriorityComparison>,
    {
        (windows_core::Interface::vtable(self).SetTrimPriorityComparison)(windows_core::Interface::as_raw(self), comparison.param().abi()).ok()
    }
    pub unsafe fn SetCompressPriorityComparison<P0>(&self, comparison: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationPriorityComparison>,
    {
        (windows_core::Interface::vtable(self).SetCompressPriorityComparison)(windows_core::Interface::as_raw(self), comparison.param().abi()).ok()
    }
    pub unsafe fn SetConcludePriorityComparison<P0>(&self, comparison: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationPriorityComparison>,
    {
        (windows_core::Interface::vtable(self).SetConcludePriorityComparison)(windows_core::Interface::as_raw(self), comparison.param().abi()).ok()
    }
    pub unsafe fn SetDefaultLongestAcceptableDelay(&self, delay: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDefaultLongestAcceptableDelay)(windows_core::Interface::as_raw(self), delay).ok()
    }
    pub unsafe fn Shutdown(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IUIAnimationManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateAnimationVariable: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ScheduleTransition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub CreateStoryboard: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FinishAllStoryboards: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub AbandonAllStoryboards: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *mut UI_ANIMATION_UPDATE_RESULT) -> windows_core::HRESULT,
    pub GetVariableFromTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStoryboardFromTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UI_ANIMATION_MANAGER_STATUS) -> windows_core::HRESULT,
    pub SetAnimationMode: unsafe extern "system" fn(*mut core::ffi::c_void, UI_ANIMATION_MODE) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetManagerEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCancelPriorityComparison: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTrimPriorityComparison: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCompressPriorityComparison: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetConcludePriorityComparison: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDefaultLongestAcceptableDelay: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIAnimationManager_Impl: Sized + windows_core::IUnknownImpl {
    fn CreateAnimationVariable(&self, initialvalue: f64) -> windows_core::Result<IUIAnimationVariable>;
    fn ScheduleTransition(&self, variable: Option<&IUIAnimationVariable>, transition: Option<&IUIAnimationTransition>, timenow: f64) -> windows_core::Result<()>;
    fn CreateStoryboard(&self) -> windows_core::Result<IUIAnimationStoryboard>;
    fn FinishAllStoryboards(&self, completiondeadline: f64) -> windows_core::Result<()>;
    fn AbandonAllStoryboards(&self) -> windows_core::Result<()>;
    fn Update(&self, timenow: f64, updateresult: *mut UI_ANIMATION_UPDATE_RESULT) -> windows_core::Result<()>;
    fn GetVariableFromTag(&self, object: Option<&windows_core::IUnknown>, id: u32) -> windows_core::Result<IUIAnimationVariable>;
    fn GetStoryboardFromTag(&self, object: Option<&windows_core::IUnknown>, id: u32) -> windows_core::Result<IUIAnimationStoryboard>;
    fn GetStatus(&self) -> windows_core::Result<UI_ANIMATION_MANAGER_STATUS>;
    fn SetAnimationMode(&self, mode: UI_ANIMATION_MODE) -> windows_core::Result<()>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
    fn SetManagerEventHandler(&self, handler: Option<&IUIAnimationManagerEventHandler>) -> windows_core::Result<()>;
    fn SetCancelPriorityComparison(&self, comparison: Option<&IUIAnimationPriorityComparison>) -> windows_core::Result<()>;
    fn SetTrimPriorityComparison(&self, comparison: Option<&IUIAnimationPriorityComparison>) -> windows_core::Result<()>;
    fn SetCompressPriorityComparison(&self, comparison: Option<&IUIAnimationPriorityComparison>) -> windows_core::Result<()>;
    fn SetConcludePriorityComparison(&self, comparison: Option<&IUIAnimationPriorityComparison>) -> windows_core::Result<()>;
    fn SetDefaultLongestAcceptableDelay(&self, delay: f64) -> windows_core::Result<()>;
    fn Shutdown(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationManager {}
impl IUIAnimationManager_Vtbl {
    pub const fn new<Identity: IUIAnimationManager_Impl, const OFFSET: isize>() -> IUIAnimationManager_Vtbl {
        unsafe extern "system" fn CreateAnimationVariable<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, initialvalue: f64, variable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationManager_Impl::CreateAnimationVariable(this, core::mem::transmute_copy(&initialvalue)) {
                Ok(ok__) => {
                    variable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScheduleTransition<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variable: *mut core::ffi::c_void, transition: *mut core::ffi::c_void, timenow: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager_Impl::ScheduleTransition(this, windows_core::from_raw_borrowed(&variable), windows_core::from_raw_borrowed(&transition), core::mem::transmute_copy(&timenow)).into()
        }
        unsafe extern "system" fn CreateStoryboard<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationManager_Impl::CreateStoryboard(this) {
                Ok(ok__) => {
                    storyboard.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinishAllStoryboards<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, completiondeadline: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager_Impl::FinishAllStoryboards(this, core::mem::transmute_copy(&completiondeadline)).into()
        }
        unsafe extern "system" fn AbandonAllStoryboards<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager_Impl::AbandonAllStoryboards(this).into()
        }
        unsafe extern "system" fn Update<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timenow: f64, updateresult: *mut UI_ANIMATION_UPDATE_RESULT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager_Impl::Update(this, core::mem::transmute_copy(&timenow), core::mem::transmute_copy(&updateresult)).into()
        }
        unsafe extern "system" fn GetVariableFromTag<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, id: u32, variable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationManager_Impl::GetVariableFromTag(this, windows_core::from_raw_borrowed(&object), core::mem::transmute_copy(&id)) {
                Ok(ok__) => {
                    variable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoryboardFromTag<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, id: u32, storyboard: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationManager_Impl::GetStoryboardFromTag(this, windows_core::from_raw_borrowed(&object), core::mem::transmute_copy(&id)) {
                Ok(ok__) => {
                    storyboard.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut UI_ANIMATION_MANAGER_STATUS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationManager_Impl::GetStatus(this) {
                Ok(ok__) => {
                    status.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAnimationMode<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: UI_ANIMATION_MODE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager_Impl::SetAnimationMode(this, core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn Pause<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager_Impl::Pause(this).into()
        }
        unsafe extern "system" fn Resume<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager_Impl::Resume(this).into()
        }
        unsafe extern "system" fn SetManagerEventHandler<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager_Impl::SetManagerEventHandler(this, windows_core::from_raw_borrowed(&handler)).into()
        }
        unsafe extern "system" fn SetCancelPriorityComparison<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, comparison: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager_Impl::SetCancelPriorityComparison(this, windows_core::from_raw_borrowed(&comparison)).into()
        }
        unsafe extern "system" fn SetTrimPriorityComparison<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, comparison: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager_Impl::SetTrimPriorityComparison(this, windows_core::from_raw_borrowed(&comparison)).into()
        }
        unsafe extern "system" fn SetCompressPriorityComparison<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, comparison: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager_Impl::SetCompressPriorityComparison(this, windows_core::from_raw_borrowed(&comparison)).into()
        }
        unsafe extern "system" fn SetConcludePriorityComparison<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, comparison: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager_Impl::SetConcludePriorityComparison(this, windows_core::from_raw_borrowed(&comparison)).into()
        }
        unsafe extern "system" fn SetDefaultLongestAcceptableDelay<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager_Impl::SetDefaultLongestAcceptableDelay(this, core::mem::transmute_copy(&delay)).into()
        }
        unsafe extern "system" fn Shutdown<Identity: IUIAnimationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager_Impl::Shutdown(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateAnimationVariable: CreateAnimationVariable::<Identity, OFFSET>,
            ScheduleTransition: ScheduleTransition::<Identity, OFFSET>,
            CreateStoryboard: CreateStoryboard::<Identity, OFFSET>,
            FinishAllStoryboards: FinishAllStoryboards::<Identity, OFFSET>,
            AbandonAllStoryboards: AbandonAllStoryboards::<Identity, OFFSET>,
            Update: Update::<Identity, OFFSET>,
            GetVariableFromTag: GetVariableFromTag::<Identity, OFFSET>,
            GetStoryboardFromTag: GetStoryboardFromTag::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            SetAnimationMode: SetAnimationMode::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
            SetManagerEventHandler: SetManagerEventHandler::<Identity, OFFSET>,
            SetCancelPriorityComparison: SetCancelPriorityComparison::<Identity, OFFSET>,
            SetTrimPriorityComparison: SetTrimPriorityComparison::<Identity, OFFSET>,
            SetCompressPriorityComparison: SetCompressPriorityComparison::<Identity, OFFSET>,
            SetConcludePriorityComparison: SetConcludePriorityComparison::<Identity, OFFSET>,
            SetDefaultLongestAcceptableDelay: SetDefaultLongestAcceptableDelay::<Identity, OFFSET>,
            Shutdown: Shutdown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationManager as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIAnimationManager2, IUIAnimationManager2_Vtbl, 0xd8b6f7d4_4109_4d3f_acee_879926968cb1);
impl core::ops::Deref for IUIAnimationManager2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAnimationManager2, windows_core::IUnknown);
impl IUIAnimationManager2 {
    pub unsafe fn CreateAnimationVectorVariable(&self, initialvalue: &[f64]) -> windows_core::Result<IUIAnimationVariable2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateAnimationVectorVariable)(windows_core::Interface::as_raw(self), core::mem::transmute(initialvalue.as_ptr()), initialvalue.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateAnimationVariable(&self, initialvalue: f64) -> windows_core::Result<IUIAnimationVariable2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateAnimationVariable)(windows_core::Interface::as_raw(self), initialvalue, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ScheduleTransition<P0, P1>(&self, variable: P0, transition: P1, timenow: f64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationVariable2>,
        P1: windows_core::Param<IUIAnimationTransition2>,
    {
        (windows_core::Interface::vtable(self).ScheduleTransition)(windows_core::Interface::as_raw(self), variable.param().abi(), transition.param().abi(), timenow).ok()
    }
    pub unsafe fn CreateStoryboard(&self) -> windows_core::Result<IUIAnimationStoryboard2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateStoryboard)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FinishAllStoryboards(&self, completiondeadline: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FinishAllStoryboards)(windows_core::Interface::as_raw(self), completiondeadline).ok()
    }
    pub unsafe fn AbandonAllStoryboards(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AbandonAllStoryboards)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Update(&self, timenow: f64, updateresult: Option<*mut UI_ANIMATION_UPDATE_RESULT>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), timenow, core::mem::transmute(updateresult.unwrap_or(core::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetVariableFromTag<P0>(&self, object: P0, id: u32) -> windows_core::Result<IUIAnimationVariable2>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVariableFromTag)(windows_core::Interface::as_raw(self), object.param().abi(), id, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetStoryboardFromTag<P0>(&self, object: P0, id: u32) -> windows_core::Result<IUIAnimationStoryboard2>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStoryboardFromTag)(windows_core::Interface::as_raw(self), object.param().abi(), id, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EstimateNextEventTime(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EstimateNextEventTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetStatus(&self) -> windows_core::Result<UI_ANIMATION_MANAGER_STATUS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAnimationMode(&self, mode: UI_ANIMATION_MODE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAnimationMode)(windows_core::Interface::as_raw(self), mode).ok()
    }
    pub unsafe fn Pause(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetManagerEventHandler<P0, P1>(&self, handler: P0, fregisterfornextanimationevent: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationManagerEventHandler2>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetManagerEventHandler)(windows_core::Interface::as_raw(self), handler.param().abi(), fregisterfornextanimationevent.param().abi()).ok()
    }
    pub unsafe fn SetCancelPriorityComparison<P0>(&self, comparison: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationPriorityComparison2>,
    {
        (windows_core::Interface::vtable(self).SetCancelPriorityComparison)(windows_core::Interface::as_raw(self), comparison.param().abi()).ok()
    }
    pub unsafe fn SetTrimPriorityComparison<P0>(&self, comparison: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationPriorityComparison2>,
    {
        (windows_core::Interface::vtable(self).SetTrimPriorityComparison)(windows_core::Interface::as_raw(self), comparison.param().abi()).ok()
    }
    pub unsafe fn SetCompressPriorityComparison<P0>(&self, comparison: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationPriorityComparison2>,
    {
        (windows_core::Interface::vtable(self).SetCompressPriorityComparison)(windows_core::Interface::as_raw(self), comparison.param().abi()).ok()
    }
    pub unsafe fn SetConcludePriorityComparison<P0>(&self, comparison: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationPriorityComparison2>,
    {
        (windows_core::Interface::vtable(self).SetConcludePriorityComparison)(windows_core::Interface::as_raw(self), comparison.param().abi()).ok()
    }
    pub unsafe fn SetDefaultLongestAcceptableDelay(&self, delay: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDefaultLongestAcceptableDelay)(windows_core::Interface::as_raw(self), delay).ok()
    }
    pub unsafe fn Shutdown(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IUIAnimationManager2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateAnimationVectorVariable: unsafe extern "system" fn(*mut core::ffi::c_void, *const f64, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateAnimationVariable: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ScheduleTransition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub CreateStoryboard: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FinishAllStoryboards: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub AbandonAllStoryboards: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *mut UI_ANIMATION_UPDATE_RESULT) -> windows_core::HRESULT,
    pub GetVariableFromTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStoryboardFromTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EstimateNextEventTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UI_ANIMATION_MANAGER_STATUS) -> windows_core::HRESULT,
    pub SetAnimationMode: unsafe extern "system" fn(*mut core::ffi::c_void, UI_ANIMATION_MODE) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetManagerEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetCancelPriorityComparison: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTrimPriorityComparison: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCompressPriorityComparison: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetConcludePriorityComparison: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDefaultLongestAcceptableDelay: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIAnimationManager2_Impl: Sized + windows_core::IUnknownImpl {
    fn CreateAnimationVectorVariable(&self, initialvalue: *const f64, cdimension: u32) -> windows_core::Result<IUIAnimationVariable2>;
    fn CreateAnimationVariable(&self, initialvalue: f64) -> windows_core::Result<IUIAnimationVariable2>;
    fn ScheduleTransition(&self, variable: Option<&IUIAnimationVariable2>, transition: Option<&IUIAnimationTransition2>, timenow: f64) -> windows_core::Result<()>;
    fn CreateStoryboard(&self) -> windows_core::Result<IUIAnimationStoryboard2>;
    fn FinishAllStoryboards(&self, completiondeadline: f64) -> windows_core::Result<()>;
    fn AbandonAllStoryboards(&self) -> windows_core::Result<()>;
    fn Update(&self, timenow: f64, updateresult: *mut UI_ANIMATION_UPDATE_RESULT) -> windows_core::Result<()>;
    fn GetVariableFromTag(&self, object: Option<&windows_core::IUnknown>, id: u32) -> windows_core::Result<IUIAnimationVariable2>;
    fn GetStoryboardFromTag(&self, object: Option<&windows_core::IUnknown>, id: u32) -> windows_core::Result<IUIAnimationStoryboard2>;
    fn EstimateNextEventTime(&self) -> windows_core::Result<f64>;
    fn GetStatus(&self) -> windows_core::Result<UI_ANIMATION_MANAGER_STATUS>;
    fn SetAnimationMode(&self, mode: UI_ANIMATION_MODE) -> windows_core::Result<()>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
    fn SetManagerEventHandler(&self, handler: Option<&IUIAnimationManagerEventHandler2>, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetCancelPriorityComparison(&self, comparison: Option<&IUIAnimationPriorityComparison2>) -> windows_core::Result<()>;
    fn SetTrimPriorityComparison(&self, comparison: Option<&IUIAnimationPriorityComparison2>) -> windows_core::Result<()>;
    fn SetCompressPriorityComparison(&self, comparison: Option<&IUIAnimationPriorityComparison2>) -> windows_core::Result<()>;
    fn SetConcludePriorityComparison(&self, comparison: Option<&IUIAnimationPriorityComparison2>) -> windows_core::Result<()>;
    fn SetDefaultLongestAcceptableDelay(&self, delay: f64) -> windows_core::Result<()>;
    fn Shutdown(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationManager2 {}
impl IUIAnimationManager2_Vtbl {
    pub const fn new<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>() -> IUIAnimationManager2_Vtbl {
        unsafe extern "system" fn CreateAnimationVectorVariable<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, initialvalue: *const f64, cdimension: u32, variable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationManager2_Impl::CreateAnimationVectorVariable(this, core::mem::transmute_copy(&initialvalue), core::mem::transmute_copy(&cdimension)) {
                Ok(ok__) => {
                    variable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAnimationVariable<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, initialvalue: f64, variable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationManager2_Impl::CreateAnimationVariable(this, core::mem::transmute_copy(&initialvalue)) {
                Ok(ok__) => {
                    variable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScheduleTransition<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variable: *mut core::ffi::c_void, transition: *mut core::ffi::c_void, timenow: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager2_Impl::ScheduleTransition(this, windows_core::from_raw_borrowed(&variable), windows_core::from_raw_borrowed(&transition), core::mem::transmute_copy(&timenow)).into()
        }
        unsafe extern "system" fn CreateStoryboard<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationManager2_Impl::CreateStoryboard(this) {
                Ok(ok__) => {
                    storyboard.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinishAllStoryboards<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, completiondeadline: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager2_Impl::FinishAllStoryboards(this, core::mem::transmute_copy(&completiondeadline)).into()
        }
        unsafe extern "system" fn AbandonAllStoryboards<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager2_Impl::AbandonAllStoryboards(this).into()
        }
        unsafe extern "system" fn Update<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timenow: f64, updateresult: *mut UI_ANIMATION_UPDATE_RESULT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager2_Impl::Update(this, core::mem::transmute_copy(&timenow), core::mem::transmute_copy(&updateresult)).into()
        }
        unsafe extern "system" fn GetVariableFromTag<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, id: u32, variable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationManager2_Impl::GetVariableFromTag(this, windows_core::from_raw_borrowed(&object), core::mem::transmute_copy(&id)) {
                Ok(ok__) => {
                    variable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStoryboardFromTag<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, id: u32, storyboard: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationManager2_Impl::GetStoryboardFromTag(this, windows_core::from_raw_borrowed(&object), core::mem::transmute_copy(&id)) {
                Ok(ok__) => {
                    storyboard.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EstimateNextEventTime<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationManager2_Impl::EstimateNextEventTime(this) {
                Ok(ok__) => {
                    seconds.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut UI_ANIMATION_MANAGER_STATUS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationManager2_Impl::GetStatus(this) {
                Ok(ok__) => {
                    status.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAnimationMode<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: UI_ANIMATION_MODE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager2_Impl::SetAnimationMode(this, core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn Pause<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager2_Impl::Pause(this).into()
        }
        unsafe extern "system" fn Resume<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager2_Impl::Resume(this).into()
        }
        unsafe extern "system" fn SetManagerEventHandler<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager2_Impl::SetManagerEventHandler(this, windows_core::from_raw_borrowed(&handler), core::mem::transmute_copy(&fregisterfornextanimationevent)).into()
        }
        unsafe extern "system" fn SetCancelPriorityComparison<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, comparison: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager2_Impl::SetCancelPriorityComparison(this, windows_core::from_raw_borrowed(&comparison)).into()
        }
        unsafe extern "system" fn SetTrimPriorityComparison<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, comparison: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager2_Impl::SetTrimPriorityComparison(this, windows_core::from_raw_borrowed(&comparison)).into()
        }
        unsafe extern "system" fn SetCompressPriorityComparison<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, comparison: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager2_Impl::SetCompressPriorityComparison(this, windows_core::from_raw_borrowed(&comparison)).into()
        }
        unsafe extern "system" fn SetConcludePriorityComparison<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, comparison: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager2_Impl::SetConcludePriorityComparison(this, windows_core::from_raw_borrowed(&comparison)).into()
        }
        unsafe extern "system" fn SetDefaultLongestAcceptableDelay<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager2_Impl::SetDefaultLongestAcceptableDelay(this, core::mem::transmute_copy(&delay)).into()
        }
        unsafe extern "system" fn Shutdown<Identity: IUIAnimationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManager2_Impl::Shutdown(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateAnimationVectorVariable: CreateAnimationVectorVariable::<Identity, OFFSET>,
            CreateAnimationVariable: CreateAnimationVariable::<Identity, OFFSET>,
            ScheduleTransition: ScheduleTransition::<Identity, OFFSET>,
            CreateStoryboard: CreateStoryboard::<Identity, OFFSET>,
            FinishAllStoryboards: FinishAllStoryboards::<Identity, OFFSET>,
            AbandonAllStoryboards: AbandonAllStoryboards::<Identity, OFFSET>,
            Update: Update::<Identity, OFFSET>,
            GetVariableFromTag: GetVariableFromTag::<Identity, OFFSET>,
            GetStoryboardFromTag: GetStoryboardFromTag::<Identity, OFFSET>,
            EstimateNextEventTime: EstimateNextEventTime::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            SetAnimationMode: SetAnimationMode::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
            SetManagerEventHandler: SetManagerEventHandler::<Identity, OFFSET>,
            SetCancelPriorityComparison: SetCancelPriorityComparison::<Identity, OFFSET>,
            SetTrimPriorityComparison: SetTrimPriorityComparison::<Identity, OFFSET>,
            SetCompressPriorityComparison: SetCompressPriorityComparison::<Identity, OFFSET>,
            SetConcludePriorityComparison: SetConcludePriorityComparison::<Identity, OFFSET>,
            SetDefaultLongestAcceptableDelay: SetDefaultLongestAcceptableDelay::<Identity, OFFSET>,
            Shutdown: Shutdown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationManager2 as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIAnimationManagerEventHandler, IUIAnimationManagerEventHandler_Vtbl, 0x783321ed_78a3_4366_b574_6af607a64788);
impl core::ops::Deref for IUIAnimationManagerEventHandler {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAnimationManagerEventHandler, windows_core::IUnknown);
impl IUIAnimationManagerEventHandler {
    pub unsafe fn OnManagerStatusChanged(&self, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnManagerStatusChanged)(windows_core::Interface::as_raw(self), newstatus, previousstatus).ok()
    }
}
#[repr(C)]
pub struct IUIAnimationManagerEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnManagerStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, UI_ANIMATION_MANAGER_STATUS, UI_ANIMATION_MANAGER_STATUS) -> windows_core::HRESULT,
}
pub trait IUIAnimationManagerEventHandler_Impl: Sized + windows_core::IUnknownImpl {
    fn OnManagerStatusChanged(&self, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationManagerEventHandler {}
impl IUIAnimationManagerEventHandler_Vtbl {
    pub const fn new<Identity: IUIAnimationManagerEventHandler_Impl, const OFFSET: isize>() -> IUIAnimationManagerEventHandler_Vtbl {
        unsafe extern "system" fn OnManagerStatusChanged<Identity: IUIAnimationManagerEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManagerEventHandler_Impl::OnManagerStatusChanged(this, core::mem::transmute_copy(&newstatus), core::mem::transmute_copy(&previousstatus)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnManagerStatusChanged: OnManagerStatusChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationManagerEventHandler as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIAnimationManagerEventHandler2, IUIAnimationManagerEventHandler2_Vtbl, 0xf6e022ba_bff3_42ec_9033_e073f33e83c3);
impl core::ops::Deref for IUIAnimationManagerEventHandler2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAnimationManagerEventHandler2, windows_core::IUnknown);
impl IUIAnimationManagerEventHandler2 {
    pub unsafe fn OnManagerStatusChanged(&self, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnManagerStatusChanged)(windows_core::Interface::as_raw(self), newstatus, previousstatus).ok()
    }
}
#[repr(C)]
pub struct IUIAnimationManagerEventHandler2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnManagerStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, UI_ANIMATION_MANAGER_STATUS, UI_ANIMATION_MANAGER_STATUS) -> windows_core::HRESULT,
}
pub trait IUIAnimationManagerEventHandler2_Impl: Sized + windows_core::IUnknownImpl {
    fn OnManagerStatusChanged(&self, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationManagerEventHandler2 {}
impl IUIAnimationManagerEventHandler2_Vtbl {
    pub const fn new<Identity: IUIAnimationManagerEventHandler2_Impl, const OFFSET: isize>() -> IUIAnimationManagerEventHandler2_Vtbl {
        unsafe extern "system" fn OnManagerStatusChanged<Identity: IUIAnimationManagerEventHandler2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newstatus: UI_ANIMATION_MANAGER_STATUS, previousstatus: UI_ANIMATION_MANAGER_STATUS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationManagerEventHandler2_Impl::OnManagerStatusChanged(this, core::mem::transmute_copy(&newstatus), core::mem::transmute_copy(&previousstatus)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnManagerStatusChanged: OnManagerStatusChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationManagerEventHandler2 as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIAnimationPrimitiveInterpolation, IUIAnimationPrimitiveInterpolation_Vtbl, 0xbab20d63_4361_45da_a24f_ab8508846b5b);
impl core::ops::Deref for IUIAnimationPrimitiveInterpolation {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAnimationPrimitiveInterpolation, windows_core::IUnknown);
impl IUIAnimationPrimitiveInterpolation {
    pub unsafe fn AddCubic(&self, dimension: u32, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddCubic)(windows_core::Interface::as_raw(self), dimension, beginoffset, constantcoefficient, linearcoefficient, quadraticcoefficient, cubiccoefficient).ok()
    }
    pub unsafe fn AddSinusoidal(&self, dimension: u32, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddSinusoidal)(windows_core::Interface::as_raw(self), dimension, beginoffset, bias, amplitude, frequency, phase).ok()
    }
}
#[repr(C)]
pub struct IUIAnimationPrimitiveInterpolation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddCubic: unsafe extern "system" fn(*mut core::ffi::c_void, u32, f64, f32, f32, f32, f32) -> windows_core::HRESULT,
    pub AddSinusoidal: unsafe extern "system" fn(*mut core::ffi::c_void, u32, f64, f32, f32, f32, f32) -> windows_core::HRESULT,
}
pub trait IUIAnimationPrimitiveInterpolation_Impl: Sized + windows_core::IUnknownImpl {
    fn AddCubic(&self, dimension: u32, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> windows_core::Result<()>;
    fn AddSinusoidal(&self, dimension: u32, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationPrimitiveInterpolation {}
impl IUIAnimationPrimitiveInterpolation_Vtbl {
    pub const fn new<Identity: IUIAnimationPrimitiveInterpolation_Impl, const OFFSET: isize>() -> IUIAnimationPrimitiveInterpolation_Vtbl {
        unsafe extern "system" fn AddCubic<Identity: IUIAnimationPrimitiveInterpolation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dimension: u32, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationPrimitiveInterpolation_Impl::AddCubic(this, core::mem::transmute_copy(&dimension), core::mem::transmute_copy(&beginoffset), core::mem::transmute_copy(&constantcoefficient), core::mem::transmute_copy(&linearcoefficient), core::mem::transmute_copy(&quadraticcoefficient), core::mem::transmute_copy(&cubiccoefficient)).into()
        }
        unsafe extern "system" fn AddSinusoidal<Identity: IUIAnimationPrimitiveInterpolation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dimension: u32, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationPrimitiveInterpolation_Impl::AddSinusoidal(this, core::mem::transmute_copy(&dimension), core::mem::transmute_copy(&beginoffset), core::mem::transmute_copy(&bias), core::mem::transmute_copy(&amplitude), core::mem::transmute_copy(&frequency), core::mem::transmute_copy(&phase)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddCubic: AddCubic::<Identity, OFFSET>,
            AddSinusoidal: AddSinusoidal::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationPrimitiveInterpolation as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIAnimationPriorityComparison, IUIAnimationPriorityComparison_Vtbl, 0x83fa9b74_5f86_4618_bc6a_a2fac19b3f44);
impl core::ops::Deref for IUIAnimationPriorityComparison {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAnimationPriorityComparison, windows_core::IUnknown);
impl IUIAnimationPriorityComparison {
    pub unsafe fn HasPriority<P0, P1>(&self, scheduledstoryboard: P0, newstoryboard: P1, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationStoryboard>,
        P1: windows_core::Param<IUIAnimationStoryboard>,
    {
        (windows_core::Interface::vtable(self).HasPriority)(windows_core::Interface::as_raw(self), scheduledstoryboard.param().abi(), newstoryboard.param().abi(), priorityeffect).ok()
    }
}
#[repr(C)]
pub struct IUIAnimationPriorityComparison_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub HasPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, UI_ANIMATION_PRIORITY_EFFECT) -> windows_core::HRESULT,
}
pub trait IUIAnimationPriorityComparison_Impl: Sized + windows_core::IUnknownImpl {
    fn HasPriority(&self, scheduledstoryboard: Option<&IUIAnimationStoryboard>, newstoryboard: Option<&IUIAnimationStoryboard>, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationPriorityComparison {}
impl IUIAnimationPriorityComparison_Vtbl {
    pub const fn new<Identity: IUIAnimationPriorityComparison_Impl, const OFFSET: isize>() -> IUIAnimationPriorityComparison_Vtbl {
        unsafe extern "system" fn HasPriority<Identity: IUIAnimationPriorityComparison_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scheduledstoryboard: *mut core::ffi::c_void, newstoryboard: *mut core::ffi::c_void, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationPriorityComparison_Impl::HasPriority(this, windows_core::from_raw_borrowed(&scheduledstoryboard), windows_core::from_raw_borrowed(&newstoryboard), core::mem::transmute_copy(&priorityeffect)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), HasPriority: HasPriority::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationPriorityComparison as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIAnimationPriorityComparison2, IUIAnimationPriorityComparison2_Vtbl, 0x5b6d7a37_4621_467c_8b05_70131de62ddb);
impl core::ops::Deref for IUIAnimationPriorityComparison2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAnimationPriorityComparison2, windows_core::IUnknown);
impl IUIAnimationPriorityComparison2 {
    pub unsafe fn HasPriority<P0, P1>(&self, scheduledstoryboard: P0, newstoryboard: P1, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationStoryboard2>,
        P1: windows_core::Param<IUIAnimationStoryboard2>,
    {
        (windows_core::Interface::vtable(self).HasPriority)(windows_core::Interface::as_raw(self), scheduledstoryboard.param().abi(), newstoryboard.param().abi(), priorityeffect).ok()
    }
}
#[repr(C)]
pub struct IUIAnimationPriorityComparison2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub HasPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, UI_ANIMATION_PRIORITY_EFFECT) -> windows_core::HRESULT,
}
pub trait IUIAnimationPriorityComparison2_Impl: Sized + windows_core::IUnknownImpl {
    fn HasPriority(&self, scheduledstoryboard: Option<&IUIAnimationStoryboard2>, newstoryboard: Option<&IUIAnimationStoryboard2>, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationPriorityComparison2 {}
impl IUIAnimationPriorityComparison2_Vtbl {
    pub const fn new<Identity: IUIAnimationPriorityComparison2_Impl, const OFFSET: isize>() -> IUIAnimationPriorityComparison2_Vtbl {
        unsafe extern "system" fn HasPriority<Identity: IUIAnimationPriorityComparison2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scheduledstoryboard: *mut core::ffi::c_void, newstoryboard: *mut core::ffi::c_void, priorityeffect: UI_ANIMATION_PRIORITY_EFFECT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationPriorityComparison2_Impl::HasPriority(this, windows_core::from_raw_borrowed(&scheduledstoryboard), windows_core::from_raw_borrowed(&newstoryboard), core::mem::transmute_copy(&priorityeffect)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), HasPriority: HasPriority::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationPriorityComparison2 as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIAnimationStoryboard, IUIAnimationStoryboard_Vtbl, 0xa8ff128f_9bf9_4af1_9e67_e5e410defb84);
impl core::ops::Deref for IUIAnimationStoryboard {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAnimationStoryboard, windows_core::IUnknown);
impl IUIAnimationStoryboard {
    pub unsafe fn AddTransition<P0, P1>(&self, variable: P0, transition: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationVariable>,
        P1: windows_core::Param<IUIAnimationTransition>,
    {
        (windows_core::Interface::vtable(self).AddTransition)(windows_core::Interface::as_raw(self), variable.param().abi(), transition.param().abi()).ok()
    }
    pub unsafe fn AddKeyframeAtOffset<P0>(&self, existingkeyframe: P0, offset: f64) -> windows_core::Result<UI_ANIMATION_KEYFRAME>
    where
        P0: windows_core::Param<UI_ANIMATION_KEYFRAME>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddKeyframeAtOffset)(windows_core::Interface::as_raw(self), existingkeyframe.param().abi(), offset, &mut result__).map(|| result__)
    }
    pub unsafe fn AddKeyframeAfterTransition<P0>(&self, transition: P0) -> windows_core::Result<UI_ANIMATION_KEYFRAME>
    where
        P0: windows_core::Param<IUIAnimationTransition>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddKeyframeAfterTransition)(windows_core::Interface::as_raw(self), transition.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn AddTransitionAtKeyframe<P0, P1, P2>(&self, variable: P0, transition: P1, startkeyframe: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationVariable>,
        P1: windows_core::Param<IUIAnimationTransition>,
        P2: windows_core::Param<UI_ANIMATION_KEYFRAME>,
    {
        (windows_core::Interface::vtable(self).AddTransitionAtKeyframe)(windows_core::Interface::as_raw(self), variable.param().abi(), transition.param().abi(), startkeyframe.param().abi()).ok()
    }
    pub unsafe fn AddTransitionBetweenKeyframes<P0, P1, P2, P3>(&self, variable: P0, transition: P1, startkeyframe: P2, endkeyframe: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationVariable>,
        P1: windows_core::Param<IUIAnimationTransition>,
        P2: windows_core::Param<UI_ANIMATION_KEYFRAME>,
        P3: windows_core::Param<UI_ANIMATION_KEYFRAME>,
    {
        (windows_core::Interface::vtable(self).AddTransitionBetweenKeyframes)(windows_core::Interface::as_raw(self), variable.param().abi(), transition.param().abi(), startkeyframe.param().abi(), endkeyframe.param().abi()).ok()
    }
    pub unsafe fn RepeatBetweenKeyframes<P0, P1>(&self, startkeyframe: P0, endkeyframe: P1, repetitioncount: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UI_ANIMATION_KEYFRAME>,
        P1: windows_core::Param<UI_ANIMATION_KEYFRAME>,
    {
        (windows_core::Interface::vtable(self).RepeatBetweenKeyframes)(windows_core::Interface::as_raw(self), startkeyframe.param().abi(), endkeyframe.param().abi(), repetitioncount).ok()
    }
    pub unsafe fn HoldVariable<P0>(&self, variable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationVariable>,
    {
        (windows_core::Interface::vtable(self).HoldVariable)(windows_core::Interface::as_raw(self), variable.param().abi()).ok()
    }
    pub unsafe fn SetLongestAcceptableDelay(&self, delay: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLongestAcceptableDelay)(windows_core::Interface::as_raw(self), delay).ok()
    }
    pub unsafe fn Schedule(&self, timenow: f64, schedulingresult: Option<*mut UI_ANIMATION_SCHEDULING_RESULT>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Schedule)(windows_core::Interface::as_raw(self), timenow, core::mem::transmute(schedulingresult.unwrap_or(core::ptr::null_mut()))).ok()
    }
    pub unsafe fn Conclude(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Conclude)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Finish(&self, completiondeadline: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Finish)(windows_core::Interface::as_raw(self), completiondeadline).ok()
    }
    pub unsafe fn Abandon(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Abandon)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetTag<P0>(&self, object: P0, id: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetTag)(windows_core::Interface::as_raw(self), object.param().abi(), id).ok()
    }
    pub unsafe fn GetTag(&self, object: Option<*mut Option<windows_core::IUnknown>>, id: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTag)(windows_core::Interface::as_raw(self), core::mem::transmute(object.unwrap_or(core::ptr::null_mut())), core::mem::transmute(id.unwrap_or(core::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetStatus(&self) -> windows_core::Result<UI_ANIMATION_STORYBOARD_STATUS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetElapsedTime(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetElapsedTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetStoryboardEventHandler<P0>(&self, handler: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationStoryboardEventHandler>,
    {
        (windows_core::Interface::vtable(self).SetStoryboardEventHandler)(windows_core::Interface::as_raw(self), handler.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IUIAnimationStoryboard_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddTransition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddKeyframeAtOffset: unsafe extern "system" fn(*mut core::ffi::c_void, UI_ANIMATION_KEYFRAME, f64, *mut UI_ANIMATION_KEYFRAME) -> windows_core::HRESULT,
    pub AddKeyframeAfterTransition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut UI_ANIMATION_KEYFRAME) -> windows_core::HRESULT,
    pub AddTransitionAtKeyframe: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, UI_ANIMATION_KEYFRAME) -> windows_core::HRESULT,
    pub AddTransitionBetweenKeyframes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, UI_ANIMATION_KEYFRAME, UI_ANIMATION_KEYFRAME) -> windows_core::HRESULT,
    pub RepeatBetweenKeyframes: unsafe extern "system" fn(*mut core::ffi::c_void, UI_ANIMATION_KEYFRAME, UI_ANIMATION_KEYFRAME, i32) -> windows_core::HRESULT,
    pub HoldVariable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLongestAcceptableDelay: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub Schedule: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *mut UI_ANIMATION_SCHEDULING_RESULT) -> windows_core::HRESULT,
    pub Conclude: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Finish: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub Abandon: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UI_ANIMATION_STORYBOARD_STATUS) -> windows_core::HRESULT,
    pub GetElapsedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetStoryboardEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIAnimationStoryboard_Impl: Sized + windows_core::IUnknownImpl {
    fn AddTransition(&self, variable: Option<&IUIAnimationVariable>, transition: Option<&IUIAnimationTransition>) -> windows_core::Result<()>;
    fn AddKeyframeAtOffset(&self, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64) -> windows_core::Result<UI_ANIMATION_KEYFRAME>;
    fn AddKeyframeAfterTransition(&self, transition: Option<&IUIAnimationTransition>) -> windows_core::Result<UI_ANIMATION_KEYFRAME>;
    fn AddTransitionAtKeyframe(&self, variable: Option<&IUIAnimationVariable>, transition: Option<&IUIAnimationTransition>, startkeyframe: UI_ANIMATION_KEYFRAME) -> windows_core::Result<()>;
    fn AddTransitionBetweenKeyframes(&self, variable: Option<&IUIAnimationVariable>, transition: Option<&IUIAnimationTransition>, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> windows_core::Result<()>;
    fn RepeatBetweenKeyframes(&self, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, repetitioncount: i32) -> windows_core::Result<()>;
    fn HoldVariable(&self, variable: Option<&IUIAnimationVariable>) -> windows_core::Result<()>;
    fn SetLongestAcceptableDelay(&self, delay: f64) -> windows_core::Result<()>;
    fn Schedule(&self, timenow: f64, schedulingresult: *mut UI_ANIMATION_SCHEDULING_RESULT) -> windows_core::Result<()>;
    fn Conclude(&self) -> windows_core::Result<()>;
    fn Finish(&self, completiondeadline: f64) -> windows_core::Result<()>;
    fn Abandon(&self) -> windows_core::Result<()>;
    fn SetTag(&self, object: Option<&windows_core::IUnknown>, id: u32) -> windows_core::Result<()>;
    fn GetTag(&self, object: *mut Option<windows_core::IUnknown>, id: *mut u32) -> windows_core::Result<()>;
    fn GetStatus(&self) -> windows_core::Result<UI_ANIMATION_STORYBOARD_STATUS>;
    fn GetElapsedTime(&self) -> windows_core::Result<f64>;
    fn SetStoryboardEventHandler(&self, handler: Option<&IUIAnimationStoryboardEventHandler>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationStoryboard {}
impl IUIAnimationStoryboard_Vtbl {
    pub const fn new<Identity: IUIAnimationStoryboard_Impl, const OFFSET: isize>() -> IUIAnimationStoryboard_Vtbl {
        unsafe extern "system" fn AddTransition<Identity: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variable: *mut core::ffi::c_void, transition: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard_Impl::AddTransition(this, windows_core::from_raw_borrowed(&variable), windows_core::from_raw_borrowed(&transition)).into()
        }
        unsafe extern "system" fn AddKeyframeAtOffset<Identity: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64, keyframe: *mut UI_ANIMATION_KEYFRAME) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationStoryboard_Impl::AddKeyframeAtOffset(this, core::mem::transmute_copy(&existingkeyframe), core::mem::transmute_copy(&offset)) {
                Ok(ok__) => {
                    keyframe.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddKeyframeAfterTransition<Identity: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transition: *mut core::ffi::c_void, keyframe: *mut UI_ANIMATION_KEYFRAME) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationStoryboard_Impl::AddKeyframeAfterTransition(this, windows_core::from_raw_borrowed(&transition)) {
                Ok(ok__) => {
                    keyframe.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTransitionAtKeyframe<Identity: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variable: *mut core::ffi::c_void, transition: *mut core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard_Impl::AddTransitionAtKeyframe(this, windows_core::from_raw_borrowed(&variable), windows_core::from_raw_borrowed(&transition), core::mem::transmute_copy(&startkeyframe)).into()
        }
        unsafe extern "system" fn AddTransitionBetweenKeyframes<Identity: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variable: *mut core::ffi::c_void, transition: *mut core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard_Impl::AddTransitionBetweenKeyframes(this, windows_core::from_raw_borrowed(&variable), windows_core::from_raw_borrowed(&transition), core::mem::transmute_copy(&startkeyframe), core::mem::transmute_copy(&endkeyframe)).into()
        }
        unsafe extern "system" fn RepeatBetweenKeyframes<Identity: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, repetitioncount: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard_Impl::RepeatBetweenKeyframes(this, core::mem::transmute_copy(&startkeyframe), core::mem::transmute_copy(&endkeyframe), core::mem::transmute_copy(&repetitioncount)).into()
        }
        unsafe extern "system" fn HoldVariable<Identity: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variable: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard_Impl::HoldVariable(this, windows_core::from_raw_borrowed(&variable)).into()
        }
        unsafe extern "system" fn SetLongestAcceptableDelay<Identity: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard_Impl::SetLongestAcceptableDelay(this, core::mem::transmute_copy(&delay)).into()
        }
        unsafe extern "system" fn Schedule<Identity: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timenow: f64, schedulingresult: *mut UI_ANIMATION_SCHEDULING_RESULT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard_Impl::Schedule(this, core::mem::transmute_copy(&timenow), core::mem::transmute_copy(&schedulingresult)).into()
        }
        unsafe extern "system" fn Conclude<Identity: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard_Impl::Conclude(this).into()
        }
        unsafe extern "system" fn Finish<Identity: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, completiondeadline: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard_Impl::Finish(this, core::mem::transmute_copy(&completiondeadline)).into()
        }
        unsafe extern "system" fn Abandon<Identity: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard_Impl::Abandon(this).into()
        }
        unsafe extern "system" fn SetTag<Identity: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, id: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard_Impl::SetTag(this, windows_core::from_raw_borrowed(&object), core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetTag<Identity: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut *mut core::ffi::c_void, id: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard_Impl::GetTag(this, core::mem::transmute_copy(&object), core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut UI_ANIMATION_STORYBOARD_STATUS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationStoryboard_Impl::GetStatus(this) {
                Ok(ok__) => {
                    status.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetElapsedTime<Identity: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, elapsedtime: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationStoryboard_Impl::GetElapsedTime(this) {
                Ok(ok__) => {
                    elapsedtime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStoryboardEventHandler<Identity: IUIAnimationStoryboard_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard_Impl::SetStoryboardEventHandler(this, windows_core::from_raw_borrowed(&handler)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddTransition: AddTransition::<Identity, OFFSET>,
            AddKeyframeAtOffset: AddKeyframeAtOffset::<Identity, OFFSET>,
            AddKeyframeAfterTransition: AddKeyframeAfterTransition::<Identity, OFFSET>,
            AddTransitionAtKeyframe: AddTransitionAtKeyframe::<Identity, OFFSET>,
            AddTransitionBetweenKeyframes: AddTransitionBetweenKeyframes::<Identity, OFFSET>,
            RepeatBetweenKeyframes: RepeatBetweenKeyframes::<Identity, OFFSET>,
            HoldVariable: HoldVariable::<Identity, OFFSET>,
            SetLongestAcceptableDelay: SetLongestAcceptableDelay::<Identity, OFFSET>,
            Schedule: Schedule::<Identity, OFFSET>,
            Conclude: Conclude::<Identity, OFFSET>,
            Finish: Finish::<Identity, OFFSET>,
            Abandon: Abandon::<Identity, OFFSET>,
            SetTag: SetTag::<Identity, OFFSET>,
            GetTag: GetTag::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            GetElapsedTime: GetElapsedTime::<Identity, OFFSET>,
            SetStoryboardEventHandler: SetStoryboardEventHandler::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationStoryboard as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIAnimationStoryboard2, IUIAnimationStoryboard2_Vtbl, 0xae289cd2_12d4_4945_9419_9e41be034df2);
impl core::ops::Deref for IUIAnimationStoryboard2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAnimationStoryboard2, windows_core::IUnknown);
impl IUIAnimationStoryboard2 {
    pub unsafe fn AddTransition<P0, P1>(&self, variable: P0, transition: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationVariable2>,
        P1: windows_core::Param<IUIAnimationTransition2>,
    {
        (windows_core::Interface::vtable(self).AddTransition)(windows_core::Interface::as_raw(self), variable.param().abi(), transition.param().abi()).ok()
    }
    pub unsafe fn AddKeyframeAtOffset<P0>(&self, existingkeyframe: P0, offset: f64) -> windows_core::Result<UI_ANIMATION_KEYFRAME>
    where
        P0: windows_core::Param<UI_ANIMATION_KEYFRAME>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddKeyframeAtOffset)(windows_core::Interface::as_raw(self), existingkeyframe.param().abi(), offset, &mut result__).map(|| result__)
    }
    pub unsafe fn AddKeyframeAfterTransition<P0>(&self, transition: P0) -> windows_core::Result<UI_ANIMATION_KEYFRAME>
    where
        P0: windows_core::Param<IUIAnimationTransition2>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddKeyframeAfterTransition)(windows_core::Interface::as_raw(self), transition.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn AddTransitionAtKeyframe<P0, P1, P2>(&self, variable: P0, transition: P1, startkeyframe: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationVariable2>,
        P1: windows_core::Param<IUIAnimationTransition2>,
        P2: windows_core::Param<UI_ANIMATION_KEYFRAME>,
    {
        (windows_core::Interface::vtable(self).AddTransitionAtKeyframe)(windows_core::Interface::as_raw(self), variable.param().abi(), transition.param().abi(), startkeyframe.param().abi()).ok()
    }
    pub unsafe fn AddTransitionBetweenKeyframes<P0, P1, P2, P3>(&self, variable: P0, transition: P1, startkeyframe: P2, endkeyframe: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationVariable2>,
        P1: windows_core::Param<IUIAnimationTransition2>,
        P2: windows_core::Param<UI_ANIMATION_KEYFRAME>,
        P3: windows_core::Param<UI_ANIMATION_KEYFRAME>,
    {
        (windows_core::Interface::vtable(self).AddTransitionBetweenKeyframes)(windows_core::Interface::as_raw(self), variable.param().abi(), transition.param().abi(), startkeyframe.param().abi(), endkeyframe.param().abi()).ok()
    }
    pub unsafe fn RepeatBetweenKeyframes<P0, P1, P2, P3>(&self, startkeyframe: P0, endkeyframe: P1, crepetition: f64, repeatmode: UI_ANIMATION_REPEAT_MODE, piterationchangehandler: P2, id: usize, fregisterfornextanimationevent: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UI_ANIMATION_KEYFRAME>,
        P1: windows_core::Param<UI_ANIMATION_KEYFRAME>,
        P2: windows_core::Param<IUIAnimationLoopIterationChangeHandler2>,
        P3: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).RepeatBetweenKeyframes)(windows_core::Interface::as_raw(self), startkeyframe.param().abi(), endkeyframe.param().abi(), crepetition, repeatmode, piterationchangehandler.param().abi(), id, fregisterfornextanimationevent.param().abi()).ok()
    }
    pub unsafe fn HoldVariable<P0>(&self, variable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationVariable2>,
    {
        (windows_core::Interface::vtable(self).HoldVariable)(windows_core::Interface::as_raw(self), variable.param().abi()).ok()
    }
    pub unsafe fn SetLongestAcceptableDelay(&self, delay: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLongestAcceptableDelay)(windows_core::Interface::as_raw(self), delay).ok()
    }
    pub unsafe fn SetSkipDuration(&self, secondsduration: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSkipDuration)(windows_core::Interface::as_raw(self), secondsduration).ok()
    }
    pub unsafe fn Schedule(&self, timenow: f64, schedulingresult: Option<*mut UI_ANIMATION_SCHEDULING_RESULT>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Schedule)(windows_core::Interface::as_raw(self), timenow, core::mem::transmute(schedulingresult.unwrap_or(core::ptr::null_mut()))).ok()
    }
    pub unsafe fn Conclude(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Conclude)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Finish(&self, completiondeadline: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Finish)(windows_core::Interface::as_raw(self), completiondeadline).ok()
    }
    pub unsafe fn Abandon(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Abandon)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetTag<P0>(&self, object: P0, id: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetTag)(windows_core::Interface::as_raw(self), object.param().abi(), id).ok()
    }
    pub unsafe fn GetTag(&self, object: Option<*mut Option<windows_core::IUnknown>>, id: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTag)(windows_core::Interface::as_raw(self), core::mem::transmute(object.unwrap_or(core::ptr::null_mut())), core::mem::transmute(id.unwrap_or(core::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetStatus(&self) -> windows_core::Result<UI_ANIMATION_STORYBOARD_STATUS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetElapsedTime(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetElapsedTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetStoryboardEventHandler<P0, P1, P2>(&self, handler: P0, fregisterstatuschangefornextanimationevent: P1, fregisterupdatefornextanimationevent: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationStoryboardEventHandler2>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetStoryboardEventHandler)(windows_core::Interface::as_raw(self), handler.param().abi(), fregisterstatuschangefornextanimationevent.param().abi(), fregisterupdatefornextanimationevent.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IUIAnimationStoryboard2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddTransition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddKeyframeAtOffset: unsafe extern "system" fn(*mut core::ffi::c_void, UI_ANIMATION_KEYFRAME, f64, *mut UI_ANIMATION_KEYFRAME) -> windows_core::HRESULT,
    pub AddKeyframeAfterTransition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut UI_ANIMATION_KEYFRAME) -> windows_core::HRESULT,
    pub AddTransitionAtKeyframe: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, UI_ANIMATION_KEYFRAME) -> windows_core::HRESULT,
    pub AddTransitionBetweenKeyframes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, UI_ANIMATION_KEYFRAME, UI_ANIMATION_KEYFRAME) -> windows_core::HRESULT,
    pub RepeatBetweenKeyframes: unsafe extern "system" fn(*mut core::ffi::c_void, UI_ANIMATION_KEYFRAME, UI_ANIMATION_KEYFRAME, f64, UI_ANIMATION_REPEAT_MODE, *mut core::ffi::c_void, usize, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub HoldVariable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLongestAcceptableDelay: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub SetSkipDuration: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub Schedule: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *mut UI_ANIMATION_SCHEDULING_RESULT) -> windows_core::HRESULT,
    pub Conclude: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Finish: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub Abandon: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UI_ANIMATION_STORYBOARD_STATUS) -> windows_core::HRESULT,
    pub GetElapsedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetStoryboardEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
pub trait IUIAnimationStoryboard2_Impl: Sized + windows_core::IUnknownImpl {
    fn AddTransition(&self, variable: Option<&IUIAnimationVariable2>, transition: Option<&IUIAnimationTransition2>) -> windows_core::Result<()>;
    fn AddKeyframeAtOffset(&self, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64) -> windows_core::Result<UI_ANIMATION_KEYFRAME>;
    fn AddKeyframeAfterTransition(&self, transition: Option<&IUIAnimationTransition2>) -> windows_core::Result<UI_ANIMATION_KEYFRAME>;
    fn AddTransitionAtKeyframe(&self, variable: Option<&IUIAnimationVariable2>, transition: Option<&IUIAnimationTransition2>, startkeyframe: UI_ANIMATION_KEYFRAME) -> windows_core::Result<()>;
    fn AddTransitionBetweenKeyframes(&self, variable: Option<&IUIAnimationVariable2>, transition: Option<&IUIAnimationTransition2>, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> windows_core::Result<()>;
    fn RepeatBetweenKeyframes(&self, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, crepetition: f64, repeatmode: UI_ANIMATION_REPEAT_MODE, piterationchangehandler: Option<&IUIAnimationLoopIterationChangeHandler2>, id: usize, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn HoldVariable(&self, variable: Option<&IUIAnimationVariable2>) -> windows_core::Result<()>;
    fn SetLongestAcceptableDelay(&self, delay: f64) -> windows_core::Result<()>;
    fn SetSkipDuration(&self, secondsduration: f64) -> windows_core::Result<()>;
    fn Schedule(&self, timenow: f64, schedulingresult: *mut UI_ANIMATION_SCHEDULING_RESULT) -> windows_core::Result<()>;
    fn Conclude(&self) -> windows_core::Result<()>;
    fn Finish(&self, completiondeadline: f64) -> windows_core::Result<()>;
    fn Abandon(&self) -> windows_core::Result<()>;
    fn SetTag(&self, object: Option<&windows_core::IUnknown>, id: u32) -> windows_core::Result<()>;
    fn GetTag(&self, object: *mut Option<windows_core::IUnknown>, id: *mut u32) -> windows_core::Result<()>;
    fn GetStatus(&self) -> windows_core::Result<UI_ANIMATION_STORYBOARD_STATUS>;
    fn GetElapsedTime(&self) -> windows_core::Result<f64>;
    fn SetStoryboardEventHandler(&self, handler: Option<&IUIAnimationStoryboardEventHandler2>, fregisterstatuschangefornextanimationevent: super::super::Foundation::BOOL, fregisterupdatefornextanimationevent: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationStoryboard2 {}
impl IUIAnimationStoryboard2_Vtbl {
    pub const fn new<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>() -> IUIAnimationStoryboard2_Vtbl {
        unsafe extern "system" fn AddTransition<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variable: *mut core::ffi::c_void, transition: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard2_Impl::AddTransition(this, windows_core::from_raw_borrowed(&variable), windows_core::from_raw_borrowed(&transition)).into()
        }
        unsafe extern "system" fn AddKeyframeAtOffset<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, existingkeyframe: UI_ANIMATION_KEYFRAME, offset: f64, keyframe: *mut UI_ANIMATION_KEYFRAME) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationStoryboard2_Impl::AddKeyframeAtOffset(this, core::mem::transmute_copy(&existingkeyframe), core::mem::transmute_copy(&offset)) {
                Ok(ok__) => {
                    keyframe.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddKeyframeAfterTransition<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transition: *mut core::ffi::c_void, keyframe: *mut UI_ANIMATION_KEYFRAME) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationStoryboard2_Impl::AddKeyframeAfterTransition(this, windows_core::from_raw_borrowed(&transition)) {
                Ok(ok__) => {
                    keyframe.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTransitionAtKeyframe<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variable: *mut core::ffi::c_void, transition: *mut core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard2_Impl::AddTransitionAtKeyframe(this, windows_core::from_raw_borrowed(&variable), windows_core::from_raw_borrowed(&transition), core::mem::transmute_copy(&startkeyframe)).into()
        }
        unsafe extern "system" fn AddTransitionBetweenKeyframes<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variable: *mut core::ffi::c_void, transition: *mut core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard2_Impl::AddTransitionBetweenKeyframes(this, windows_core::from_raw_borrowed(&variable), windows_core::from_raw_borrowed(&transition), core::mem::transmute_copy(&startkeyframe), core::mem::transmute_copy(&endkeyframe)).into()
        }
        unsafe extern "system" fn RepeatBetweenKeyframes<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startkeyframe: UI_ANIMATION_KEYFRAME, endkeyframe: UI_ANIMATION_KEYFRAME, crepetition: f64, repeatmode: UI_ANIMATION_REPEAT_MODE, piterationchangehandler: *mut core::ffi::c_void, id: usize, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard2_Impl::RepeatBetweenKeyframes(this, core::mem::transmute_copy(&startkeyframe), core::mem::transmute_copy(&endkeyframe), core::mem::transmute_copy(&crepetition), core::mem::transmute_copy(&repeatmode), windows_core::from_raw_borrowed(&piterationchangehandler), core::mem::transmute_copy(&id), core::mem::transmute_copy(&fregisterfornextanimationevent)).into()
        }
        unsafe extern "system" fn HoldVariable<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variable: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard2_Impl::HoldVariable(this, windows_core::from_raw_borrowed(&variable)).into()
        }
        unsafe extern "system" fn SetLongestAcceptableDelay<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard2_Impl::SetLongestAcceptableDelay(this, core::mem::transmute_copy(&delay)).into()
        }
        unsafe extern "system" fn SetSkipDuration<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, secondsduration: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard2_Impl::SetSkipDuration(this, core::mem::transmute_copy(&secondsduration)).into()
        }
        unsafe extern "system" fn Schedule<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timenow: f64, schedulingresult: *mut UI_ANIMATION_SCHEDULING_RESULT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard2_Impl::Schedule(this, core::mem::transmute_copy(&timenow), core::mem::transmute_copy(&schedulingresult)).into()
        }
        unsafe extern "system" fn Conclude<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard2_Impl::Conclude(this).into()
        }
        unsafe extern "system" fn Finish<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, completiondeadline: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard2_Impl::Finish(this, core::mem::transmute_copy(&completiondeadline)).into()
        }
        unsafe extern "system" fn Abandon<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard2_Impl::Abandon(this).into()
        }
        unsafe extern "system" fn SetTag<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, id: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard2_Impl::SetTag(this, windows_core::from_raw_borrowed(&object), core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetTag<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut *mut core::ffi::c_void, id: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard2_Impl::GetTag(this, core::mem::transmute_copy(&object), core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut UI_ANIMATION_STORYBOARD_STATUS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationStoryboard2_Impl::GetStatus(this) {
                Ok(ok__) => {
                    status.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetElapsedTime<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, elapsedtime: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationStoryboard2_Impl::GetElapsedTime(this) {
                Ok(ok__) => {
                    elapsedtime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStoryboardEventHandler<Identity: IUIAnimationStoryboard2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, fregisterstatuschangefornextanimationevent: super::super::Foundation::BOOL, fregisterupdatefornextanimationevent: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboard2_Impl::SetStoryboardEventHandler(this, windows_core::from_raw_borrowed(&handler), core::mem::transmute_copy(&fregisterstatuschangefornextanimationevent), core::mem::transmute_copy(&fregisterupdatefornextanimationevent)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddTransition: AddTransition::<Identity, OFFSET>,
            AddKeyframeAtOffset: AddKeyframeAtOffset::<Identity, OFFSET>,
            AddKeyframeAfterTransition: AddKeyframeAfterTransition::<Identity, OFFSET>,
            AddTransitionAtKeyframe: AddTransitionAtKeyframe::<Identity, OFFSET>,
            AddTransitionBetweenKeyframes: AddTransitionBetweenKeyframes::<Identity, OFFSET>,
            RepeatBetweenKeyframes: RepeatBetweenKeyframes::<Identity, OFFSET>,
            HoldVariable: HoldVariable::<Identity, OFFSET>,
            SetLongestAcceptableDelay: SetLongestAcceptableDelay::<Identity, OFFSET>,
            SetSkipDuration: SetSkipDuration::<Identity, OFFSET>,
            Schedule: Schedule::<Identity, OFFSET>,
            Conclude: Conclude::<Identity, OFFSET>,
            Finish: Finish::<Identity, OFFSET>,
            Abandon: Abandon::<Identity, OFFSET>,
            SetTag: SetTag::<Identity, OFFSET>,
            GetTag: GetTag::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            GetElapsedTime: GetElapsedTime::<Identity, OFFSET>,
            SetStoryboardEventHandler: SetStoryboardEventHandler::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationStoryboard2 as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIAnimationStoryboardEventHandler, IUIAnimationStoryboardEventHandler_Vtbl, 0x3d5c9008_ec7c_4364_9f8a_9af3c58cbae6);
impl core::ops::Deref for IUIAnimationStoryboardEventHandler {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAnimationStoryboardEventHandler, windows_core::IUnknown);
impl IUIAnimationStoryboardEventHandler {
    pub unsafe fn OnStoryboardStatusChanged<P0>(&self, storyboard: P0, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationStoryboard>,
    {
        (windows_core::Interface::vtable(self).OnStoryboardStatusChanged)(windows_core::Interface::as_raw(self), storyboard.param().abi(), newstatus, previousstatus).ok()
    }
    pub unsafe fn OnStoryboardUpdated<P0>(&self, storyboard: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationStoryboard>,
    {
        (windows_core::Interface::vtable(self).OnStoryboardUpdated)(windows_core::Interface::as_raw(self), storyboard.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IUIAnimationStoryboardEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnStoryboardStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, UI_ANIMATION_STORYBOARD_STATUS, UI_ANIMATION_STORYBOARD_STATUS) -> windows_core::HRESULT,
    pub OnStoryboardUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIAnimationStoryboardEventHandler_Impl: Sized + windows_core::IUnknownImpl {
    fn OnStoryboardStatusChanged(&self, storyboard: Option<&IUIAnimationStoryboard>, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> windows_core::Result<()>;
    fn OnStoryboardUpdated(&self, storyboard: Option<&IUIAnimationStoryboard>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationStoryboardEventHandler {}
impl IUIAnimationStoryboardEventHandler_Vtbl {
    pub const fn new<Identity: IUIAnimationStoryboardEventHandler_Impl, const OFFSET: isize>() -> IUIAnimationStoryboardEventHandler_Vtbl {
        unsafe extern "system" fn OnStoryboardStatusChanged<Identity: IUIAnimationStoryboardEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut core::ffi::c_void, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboardEventHandler_Impl::OnStoryboardStatusChanged(this, windows_core::from_raw_borrowed(&storyboard), core::mem::transmute_copy(&newstatus), core::mem::transmute_copy(&previousstatus)).into()
        }
        unsafe extern "system" fn OnStoryboardUpdated<Identity: IUIAnimationStoryboardEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboardEventHandler_Impl::OnStoryboardUpdated(this, windows_core::from_raw_borrowed(&storyboard)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStoryboardStatusChanged: OnStoryboardStatusChanged::<Identity, OFFSET>,
            OnStoryboardUpdated: OnStoryboardUpdated::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationStoryboardEventHandler as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIAnimationStoryboardEventHandler2, IUIAnimationStoryboardEventHandler2_Vtbl, 0xbac5f55a_ba7c_414c_b599_fbf850f553c6);
impl core::ops::Deref for IUIAnimationStoryboardEventHandler2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAnimationStoryboardEventHandler2, windows_core::IUnknown);
impl IUIAnimationStoryboardEventHandler2 {
    pub unsafe fn OnStoryboardStatusChanged<P0>(&self, storyboard: P0, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationStoryboard2>,
    {
        (windows_core::Interface::vtable(self).OnStoryboardStatusChanged)(windows_core::Interface::as_raw(self), storyboard.param().abi(), newstatus, previousstatus).ok()
    }
    pub unsafe fn OnStoryboardUpdated<P0>(&self, storyboard: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationStoryboard2>,
    {
        (windows_core::Interface::vtable(self).OnStoryboardUpdated)(windows_core::Interface::as_raw(self), storyboard.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IUIAnimationStoryboardEventHandler2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnStoryboardStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, UI_ANIMATION_STORYBOARD_STATUS, UI_ANIMATION_STORYBOARD_STATUS) -> windows_core::HRESULT,
    pub OnStoryboardUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIAnimationStoryboardEventHandler2_Impl: Sized + windows_core::IUnknownImpl {
    fn OnStoryboardStatusChanged(&self, storyboard: Option<&IUIAnimationStoryboard2>, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> windows_core::Result<()>;
    fn OnStoryboardUpdated(&self, storyboard: Option<&IUIAnimationStoryboard2>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationStoryboardEventHandler2 {}
impl IUIAnimationStoryboardEventHandler2_Vtbl {
    pub const fn new<Identity: IUIAnimationStoryboardEventHandler2_Impl, const OFFSET: isize>() -> IUIAnimationStoryboardEventHandler2_Vtbl {
        unsafe extern "system" fn OnStoryboardStatusChanged<Identity: IUIAnimationStoryboardEventHandler2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut core::ffi::c_void, newstatus: UI_ANIMATION_STORYBOARD_STATUS, previousstatus: UI_ANIMATION_STORYBOARD_STATUS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboardEventHandler2_Impl::OnStoryboardStatusChanged(this, windows_core::from_raw_borrowed(&storyboard), core::mem::transmute_copy(&newstatus), core::mem::transmute_copy(&previousstatus)).into()
        }
        unsafe extern "system" fn OnStoryboardUpdated<Identity: IUIAnimationStoryboardEventHandler2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationStoryboardEventHandler2_Impl::OnStoryboardUpdated(this, windows_core::from_raw_borrowed(&storyboard)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStoryboardStatusChanged: OnStoryboardStatusChanged::<Identity, OFFSET>,
            OnStoryboardUpdated: OnStoryboardUpdated::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationStoryboardEventHandler2 as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIAnimationTimer, IUIAnimationTimer_Vtbl, 0x6b0efad1_a053_41d6_9085_33a689144665);
impl core::ops::Deref for IUIAnimationTimer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAnimationTimer, windows_core::IUnknown);
impl IUIAnimationTimer {
    pub unsafe fn SetTimerUpdateHandler<P0>(&self, updatehandler: P0, idlebehavior: UI_ANIMATION_IDLE_BEHAVIOR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationTimerUpdateHandler>,
    {
        (windows_core::Interface::vtable(self).SetTimerUpdateHandler)(windows_core::Interface::as_raw(self), updatehandler.param().abi(), idlebehavior).ok()
    }
    pub unsafe fn SetTimerEventHandler<P0>(&self, handler: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationTimerEventHandler>,
    {
        (windows_core::Interface::vtable(self).SetTimerEventHandler)(windows_core::Interface::as_raw(self), handler.param().abi()).ok()
    }
    pub unsafe fn Enable(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Enable)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Disable(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Disable)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsEnabled(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsEnabled)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetTime(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFrameRateThreshold(&self, framespersecond: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFrameRateThreshold)(windows_core::Interface::as_raw(self), framespersecond).ok()
    }
}
#[repr(C)]
pub struct IUIAnimationTimer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetTimerUpdateHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, UI_ANIMATION_IDLE_BEHAVIOR) -> windows_core::HRESULT,
    pub SetTimerEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Enable: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disable: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetFrameRateThreshold: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IUIAnimationTimer_Impl: Sized + windows_core::IUnknownImpl {
    fn SetTimerUpdateHandler(&self, updatehandler: Option<&IUIAnimationTimerUpdateHandler>, idlebehavior: UI_ANIMATION_IDLE_BEHAVIOR) -> windows_core::Result<()>;
    fn SetTimerEventHandler(&self, handler: Option<&IUIAnimationTimerEventHandler>) -> windows_core::Result<()>;
    fn Enable(&self) -> windows_core::Result<()>;
    fn Disable(&self) -> windows_core::Result<()>;
    fn IsEnabled(&self) -> windows_core::Result<()>;
    fn GetTime(&self) -> windows_core::Result<f64>;
    fn SetFrameRateThreshold(&self, framespersecond: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationTimer {}
impl IUIAnimationTimer_Vtbl {
    pub const fn new<Identity: IUIAnimationTimer_Impl, const OFFSET: isize>() -> IUIAnimationTimer_Vtbl {
        unsafe extern "system" fn SetTimerUpdateHandler<Identity: IUIAnimationTimer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, updatehandler: *mut core::ffi::c_void, idlebehavior: UI_ANIMATION_IDLE_BEHAVIOR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTimer_Impl::SetTimerUpdateHandler(this, windows_core::from_raw_borrowed(&updatehandler), core::mem::transmute_copy(&idlebehavior)).into()
        }
        unsafe extern "system" fn SetTimerEventHandler<Identity: IUIAnimationTimer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTimer_Impl::SetTimerEventHandler(this, windows_core::from_raw_borrowed(&handler)).into()
        }
        unsafe extern "system" fn Enable<Identity: IUIAnimationTimer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTimer_Impl::Enable(this).into()
        }
        unsafe extern "system" fn Disable<Identity: IUIAnimationTimer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTimer_Impl::Disable(this).into()
        }
        unsafe extern "system" fn IsEnabled<Identity: IUIAnimationTimer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTimer_Impl::IsEnabled(this).into()
        }
        unsafe extern "system" fn GetTime<Identity: IUIAnimationTimer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTimer_Impl::GetTime(this) {
                Ok(ok__) => {
                    seconds.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrameRateThreshold<Identity: IUIAnimationTimer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, framespersecond: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTimer_Impl::SetFrameRateThreshold(this, core::mem::transmute_copy(&framespersecond)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetTimerUpdateHandler: SetTimerUpdateHandler::<Identity, OFFSET>,
            SetTimerEventHandler: SetTimerEventHandler::<Identity, OFFSET>,
            Enable: Enable::<Identity, OFFSET>,
            Disable: Disable::<Identity, OFFSET>,
            IsEnabled: IsEnabled::<Identity, OFFSET>,
            GetTime: GetTime::<Identity, OFFSET>,
            SetFrameRateThreshold: SetFrameRateThreshold::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationTimer as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIAnimationTimerClientEventHandler, IUIAnimationTimerClientEventHandler_Vtbl, 0xbedb4db6_94fa_4bfb_a47f_ef2d9e408c25);
impl core::ops::Deref for IUIAnimationTimerClientEventHandler {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAnimationTimerClientEventHandler, windows_core::IUnknown);
impl IUIAnimationTimerClientEventHandler {
    pub unsafe fn OnTimerClientStatusChanged(&self, newstatus: UI_ANIMATION_TIMER_CLIENT_STATUS, previousstatus: UI_ANIMATION_TIMER_CLIENT_STATUS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnTimerClientStatusChanged)(windows_core::Interface::as_raw(self), newstatus, previousstatus).ok()
    }
}
#[repr(C)]
pub struct IUIAnimationTimerClientEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnTimerClientStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, UI_ANIMATION_TIMER_CLIENT_STATUS, UI_ANIMATION_TIMER_CLIENT_STATUS) -> windows_core::HRESULT,
}
pub trait IUIAnimationTimerClientEventHandler_Impl: Sized + windows_core::IUnknownImpl {
    fn OnTimerClientStatusChanged(&self, newstatus: UI_ANIMATION_TIMER_CLIENT_STATUS, previousstatus: UI_ANIMATION_TIMER_CLIENT_STATUS) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationTimerClientEventHandler {}
impl IUIAnimationTimerClientEventHandler_Vtbl {
    pub const fn new<Identity: IUIAnimationTimerClientEventHandler_Impl, const OFFSET: isize>() -> IUIAnimationTimerClientEventHandler_Vtbl {
        unsafe extern "system" fn OnTimerClientStatusChanged<Identity: IUIAnimationTimerClientEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newstatus: UI_ANIMATION_TIMER_CLIENT_STATUS, previousstatus: UI_ANIMATION_TIMER_CLIENT_STATUS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTimerClientEventHandler_Impl::OnTimerClientStatusChanged(this, core::mem::transmute_copy(&newstatus), core::mem::transmute_copy(&previousstatus)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnTimerClientStatusChanged: OnTimerClientStatusChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationTimerClientEventHandler as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIAnimationTimerEventHandler, IUIAnimationTimerEventHandler_Vtbl, 0x274a7dea_d771_4095_abbd_8df7abd23ce3);
impl core::ops::Deref for IUIAnimationTimerEventHandler {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAnimationTimerEventHandler, windows_core::IUnknown);
impl IUIAnimationTimerEventHandler {
    pub unsafe fn OnPreUpdate(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnPreUpdate)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnPostUpdate(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnPostUpdate)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnRenderingTooSlow(&self, framespersecond: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnRenderingTooSlow)(windows_core::Interface::as_raw(self), framespersecond).ok()
    }
}
#[repr(C)]
pub struct IUIAnimationTimerEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnPreUpdate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnPostUpdate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnRenderingTooSlow: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IUIAnimationTimerEventHandler_Impl: Sized + windows_core::IUnknownImpl {
    fn OnPreUpdate(&self) -> windows_core::Result<()>;
    fn OnPostUpdate(&self) -> windows_core::Result<()>;
    fn OnRenderingTooSlow(&self, framespersecond: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationTimerEventHandler {}
impl IUIAnimationTimerEventHandler_Vtbl {
    pub const fn new<Identity: IUIAnimationTimerEventHandler_Impl, const OFFSET: isize>() -> IUIAnimationTimerEventHandler_Vtbl {
        unsafe extern "system" fn OnPreUpdate<Identity: IUIAnimationTimerEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTimerEventHandler_Impl::OnPreUpdate(this).into()
        }
        unsafe extern "system" fn OnPostUpdate<Identity: IUIAnimationTimerEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTimerEventHandler_Impl::OnPostUpdate(this).into()
        }
        unsafe extern "system" fn OnRenderingTooSlow<Identity: IUIAnimationTimerEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, framespersecond: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTimerEventHandler_Impl::OnRenderingTooSlow(this, core::mem::transmute_copy(&framespersecond)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnPreUpdate: OnPreUpdate::<Identity, OFFSET>,
            OnPostUpdate: OnPostUpdate::<Identity, OFFSET>,
            OnRenderingTooSlow: OnRenderingTooSlow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationTimerEventHandler as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIAnimationTimerUpdateHandler, IUIAnimationTimerUpdateHandler_Vtbl, 0x195509b7_5d5e_4e3e_b278_ee3759b367ad);
impl core::ops::Deref for IUIAnimationTimerUpdateHandler {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAnimationTimerUpdateHandler, windows_core::IUnknown);
impl IUIAnimationTimerUpdateHandler {
    pub unsafe fn OnUpdate(&self, timenow: f64) -> windows_core::Result<UI_ANIMATION_UPDATE_RESULT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OnUpdate)(windows_core::Interface::as_raw(self), timenow, &mut result__).map(|| result__)
    }
    pub unsafe fn SetTimerClientEventHandler<P0>(&self, handler: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationTimerClientEventHandler>,
    {
        (windows_core::Interface::vtable(self).SetTimerClientEventHandler)(windows_core::Interface::as_raw(self), handler.param().abi()).ok()
    }
    pub unsafe fn ClearTimerClientEventHandler(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ClearTimerClientEventHandler)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IUIAnimationTimerUpdateHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *mut UI_ANIMATION_UPDATE_RESULT) -> windows_core::HRESULT,
    pub SetTimerClientEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearTimerClientEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIAnimationTimerUpdateHandler_Impl: Sized + windows_core::IUnknownImpl {
    fn OnUpdate(&self, timenow: f64) -> windows_core::Result<UI_ANIMATION_UPDATE_RESULT>;
    fn SetTimerClientEventHandler(&self, handler: Option<&IUIAnimationTimerClientEventHandler>) -> windows_core::Result<()>;
    fn ClearTimerClientEventHandler(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationTimerUpdateHandler {}
impl IUIAnimationTimerUpdateHandler_Vtbl {
    pub const fn new<Identity: IUIAnimationTimerUpdateHandler_Impl, const OFFSET: isize>() -> IUIAnimationTimerUpdateHandler_Vtbl {
        unsafe extern "system" fn OnUpdate<Identity: IUIAnimationTimerUpdateHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timenow: f64, result: *mut UI_ANIMATION_UPDATE_RESULT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTimerUpdateHandler_Impl::OnUpdate(this, core::mem::transmute_copy(&timenow)) {
                Ok(ok__) => {
                    result.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimerClientEventHandler<Identity: IUIAnimationTimerUpdateHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTimerUpdateHandler_Impl::SetTimerClientEventHandler(this, windows_core::from_raw_borrowed(&handler)).into()
        }
        unsafe extern "system" fn ClearTimerClientEventHandler<Identity: IUIAnimationTimerUpdateHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTimerUpdateHandler_Impl::ClearTimerClientEventHandler(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnUpdate: OnUpdate::<Identity, OFFSET>,
            SetTimerClientEventHandler: SetTimerClientEventHandler::<Identity, OFFSET>,
            ClearTimerClientEventHandler: ClearTimerClientEventHandler::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationTimerUpdateHandler as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIAnimationTransition, IUIAnimationTransition_Vtbl, 0xdc6ce252_f731_41cf_b610_614b6ca049ad);
impl core::ops::Deref for IUIAnimationTransition {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAnimationTransition, windows_core::IUnknown);
impl IUIAnimationTransition {
    pub unsafe fn SetInitialValue(&self, value: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetInitialValue)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn SetInitialVelocity(&self, velocity: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetInitialVelocity)(windows_core::Interface::as_raw(self), velocity).ok()
    }
    pub unsafe fn IsDurationKnown(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsDurationKnown)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetDuration(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDuration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IUIAnimationTransition_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetInitialValue: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub SetInitialVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub IsDurationKnown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
pub trait IUIAnimationTransition_Impl: Sized + windows_core::IUnknownImpl {
    fn SetInitialValue(&self, value: f64) -> windows_core::Result<()>;
    fn SetInitialVelocity(&self, velocity: f64) -> windows_core::Result<()>;
    fn IsDurationKnown(&self) -> windows_core::Result<()>;
    fn GetDuration(&self) -> windows_core::Result<f64>;
}
impl windows_core::RuntimeName for IUIAnimationTransition {}
impl IUIAnimationTransition_Vtbl {
    pub const fn new<Identity: IUIAnimationTransition_Impl, const OFFSET: isize>() -> IUIAnimationTransition_Vtbl {
        unsafe extern "system" fn SetInitialValue<Identity: IUIAnimationTransition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTransition_Impl::SetInitialValue(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetInitialVelocity<Identity: IUIAnimationTransition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, velocity: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTransition_Impl::SetInitialVelocity(this, core::mem::transmute_copy(&velocity)).into()
        }
        unsafe extern "system" fn IsDurationKnown<Identity: IUIAnimationTransition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTransition_Impl::IsDurationKnown(this).into()
        }
        unsafe extern "system" fn GetDuration<Identity: IUIAnimationTransition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransition_Impl::GetDuration(this) {
                Ok(ok__) => {
                    duration.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetInitialValue: SetInitialValue::<Identity, OFFSET>,
            SetInitialVelocity: SetInitialVelocity::<Identity, OFFSET>,
            IsDurationKnown: IsDurationKnown::<Identity, OFFSET>,
            GetDuration: GetDuration::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationTransition as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIAnimationTransition2, IUIAnimationTransition2_Vtbl, 0x62ff9123_a85a_4e9b_a218_435a93e268fd);
impl core::ops::Deref for IUIAnimationTransition2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAnimationTransition2, windows_core::IUnknown);
impl IUIAnimationTransition2 {
    pub unsafe fn GetDimension(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDimension)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetInitialValue(&self, value: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetInitialValue)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn SetInitialVectorValue(&self, value: &[f64]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetInitialVectorValue)(windows_core::Interface::as_raw(self), core::mem::transmute(value.as_ptr()), value.len().try_into().unwrap()).ok()
    }
    pub unsafe fn SetInitialVelocity(&self, velocity: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetInitialVelocity)(windows_core::Interface::as_raw(self), velocity).ok()
    }
    pub unsafe fn SetInitialVectorVelocity(&self, velocity: &[f64]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetInitialVectorVelocity)(windows_core::Interface::as_raw(self), core::mem::transmute(velocity.as_ptr()), velocity.len().try_into().unwrap()).ok()
    }
    pub unsafe fn IsDurationKnown(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsDurationKnown)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetDuration(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDuration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IUIAnimationTransition2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDimension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetInitialValue: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub SetInitialVectorValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const f64, u32) -> windows_core::HRESULT,
    pub SetInitialVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub SetInitialVectorVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, *const f64, u32) -> windows_core::HRESULT,
    pub IsDurationKnown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
pub trait IUIAnimationTransition2_Impl: Sized + windows_core::IUnknownImpl {
    fn GetDimension(&self) -> windows_core::Result<u32>;
    fn SetInitialValue(&self, value: f64) -> windows_core::Result<()>;
    fn SetInitialVectorValue(&self, value: *const f64, cdimension: u32) -> windows_core::Result<()>;
    fn SetInitialVelocity(&self, velocity: f64) -> windows_core::Result<()>;
    fn SetInitialVectorVelocity(&self, velocity: *const f64, cdimension: u32) -> windows_core::Result<()>;
    fn IsDurationKnown(&self) -> windows_core::Result<()>;
    fn GetDuration(&self) -> windows_core::Result<f64>;
}
impl windows_core::RuntimeName for IUIAnimationTransition2 {}
impl IUIAnimationTransition2_Vtbl {
    pub const fn new<Identity: IUIAnimationTransition2_Impl, const OFFSET: isize>() -> IUIAnimationTransition2_Vtbl {
        unsafe extern "system" fn GetDimension<Identity: IUIAnimationTransition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dimension: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransition2_Impl::GetDimension(this) {
                Ok(ok__) => {
                    dimension.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialValue<Identity: IUIAnimationTransition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTransition2_Impl::SetInitialValue(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetInitialVectorValue<Identity: IUIAnimationTransition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *const f64, cdimension: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTransition2_Impl::SetInitialVectorValue(this, core::mem::transmute_copy(&value), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn SetInitialVelocity<Identity: IUIAnimationTransition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, velocity: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTransition2_Impl::SetInitialVelocity(this, core::mem::transmute_copy(&velocity)).into()
        }
        unsafe extern "system" fn SetInitialVectorVelocity<Identity: IUIAnimationTransition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, velocity: *const f64, cdimension: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTransition2_Impl::SetInitialVectorVelocity(this, core::mem::transmute_copy(&velocity), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn IsDurationKnown<Identity: IUIAnimationTransition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationTransition2_Impl::IsDurationKnown(this).into()
        }
        unsafe extern "system" fn GetDuration<Identity: IUIAnimationTransition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransition2_Impl::GetDuration(this) {
                Ok(ok__) => {
                    duration.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDimension: GetDimension::<Identity, OFFSET>,
            SetInitialValue: SetInitialValue::<Identity, OFFSET>,
            SetInitialVectorValue: SetInitialVectorValue::<Identity, OFFSET>,
            SetInitialVelocity: SetInitialVelocity::<Identity, OFFSET>,
            SetInitialVectorVelocity: SetInitialVectorVelocity::<Identity, OFFSET>,
            IsDurationKnown: IsDurationKnown::<Identity, OFFSET>,
            GetDuration: GetDuration::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationTransition2 as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIAnimationTransitionFactory, IUIAnimationTransitionFactory_Vtbl, 0xfcd91e03_3e3b_45ad_bbb1_6dfc8153743d);
impl core::ops::Deref for IUIAnimationTransitionFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAnimationTransitionFactory, windows_core::IUnknown);
impl IUIAnimationTransitionFactory {
    pub unsafe fn CreateTransition<P0>(&self, interpolator: P0) -> windows_core::Result<IUIAnimationTransition>
    where
        P0: windows_core::Param<IUIAnimationInterpolator>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTransition)(windows_core::Interface::as_raw(self), interpolator.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUIAnimationTransitionFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateTransition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIAnimationTransitionFactory_Impl: Sized + windows_core::IUnknownImpl {
    fn CreateTransition(&self, interpolator: Option<&IUIAnimationInterpolator>) -> windows_core::Result<IUIAnimationTransition>;
}
impl windows_core::RuntimeName for IUIAnimationTransitionFactory {}
impl IUIAnimationTransitionFactory_Vtbl {
    pub const fn new<Identity: IUIAnimationTransitionFactory_Impl, const OFFSET: isize>() -> IUIAnimationTransitionFactory_Vtbl {
        unsafe extern "system" fn CreateTransition<Identity: IUIAnimationTransitionFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interpolator: *mut core::ffi::c_void, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionFactory_Impl::CreateTransition(this, windows_core::from_raw_borrowed(&interpolator)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateTransition: CreateTransition::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationTransitionFactory as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIAnimationTransitionFactory2, IUIAnimationTransitionFactory2_Vtbl, 0x937d4916_c1a6_42d5_88d8_30344d6efe31);
impl core::ops::Deref for IUIAnimationTransitionFactory2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAnimationTransitionFactory2, windows_core::IUnknown);
impl IUIAnimationTransitionFactory2 {
    pub unsafe fn CreateTransition<P0>(&self, interpolator: P0) -> windows_core::Result<IUIAnimationTransition2>
    where
        P0: windows_core::Param<IUIAnimationInterpolator2>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTransition)(windows_core::Interface::as_raw(self), interpolator.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUIAnimationTransitionFactory2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateTransition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIAnimationTransitionFactory2_Impl: Sized + windows_core::IUnknownImpl {
    fn CreateTransition(&self, interpolator: Option<&IUIAnimationInterpolator2>) -> windows_core::Result<IUIAnimationTransition2>;
}
impl windows_core::RuntimeName for IUIAnimationTransitionFactory2 {}
impl IUIAnimationTransitionFactory2_Vtbl {
    pub const fn new<Identity: IUIAnimationTransitionFactory2_Impl, const OFFSET: isize>() -> IUIAnimationTransitionFactory2_Vtbl {
        unsafe extern "system" fn CreateTransition<Identity: IUIAnimationTransitionFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interpolator: *mut core::ffi::c_void, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionFactory2_Impl::CreateTransition(this, windows_core::from_raw_borrowed(&interpolator)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateTransition: CreateTransition::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationTransitionFactory2 as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIAnimationTransitionLibrary, IUIAnimationTransitionLibrary_Vtbl, 0xca5a14b1_d24f_48b8_8fe4_c78169ba954e);
impl core::ops::Deref for IUIAnimationTransitionLibrary {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAnimationTransitionLibrary, windows_core::IUnknown);
impl IUIAnimationTransitionLibrary {
    pub unsafe fn CreateInstantaneousTransition(&self, finalvalue: f64) -> windows_core::Result<IUIAnimationTransition> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateInstantaneousTransition)(windows_core::Interface::as_raw(self), finalvalue, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateConstantTransition(&self, duration: f64) -> windows_core::Result<IUIAnimationTransition> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateConstantTransition)(windows_core::Interface::as_raw(self), duration, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateDiscreteTransition(&self, delay: f64, finalvalue: f64, hold: f64) -> windows_core::Result<IUIAnimationTransition> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDiscreteTransition)(windows_core::Interface::as_raw(self), delay, finalvalue, hold, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateLinearTransition(&self, duration: f64, finalvalue: f64) -> windows_core::Result<IUIAnimationTransition> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateLinearTransition)(windows_core::Interface::as_raw(self), duration, finalvalue, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateLinearTransitionFromSpeed(&self, speed: f64, finalvalue: f64) -> windows_core::Result<IUIAnimationTransition> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateLinearTransitionFromSpeed)(windows_core::Interface::as_raw(self), speed, finalvalue, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateSinusoidalTransitionFromVelocity(&self, duration: f64, period: f64) -> windows_core::Result<IUIAnimationTransition> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSinusoidalTransitionFromVelocity)(windows_core::Interface::as_raw(self), duration, period, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateSinusoidalTransitionFromRange(&self, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE) -> windows_core::Result<IUIAnimationTransition> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSinusoidalTransitionFromRange)(windows_core::Interface::as_raw(self), duration, minimumvalue, maximumvalue, period, slope, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateAccelerateDecelerateTransition(&self, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64) -> windows_core::Result<IUIAnimationTransition> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateAccelerateDecelerateTransition)(windows_core::Interface::as_raw(self), duration, finalvalue, accelerationratio, decelerationratio, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateReversalTransition(&self, duration: f64) -> windows_core::Result<IUIAnimationTransition> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateReversalTransition)(windows_core::Interface::as_raw(self), duration, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateCubicTransition(&self, duration: f64, finalvalue: f64, finalvelocity: f64) -> windows_core::Result<IUIAnimationTransition> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateCubicTransition)(windows_core::Interface::as_raw(self), duration, finalvalue, finalvelocity, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateSmoothStopTransition(&self, maximumduration: f64, finalvalue: f64) -> windows_core::Result<IUIAnimationTransition> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSmoothStopTransition)(windows_core::Interface::as_raw(self), maximumduration, finalvalue, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateParabolicTransitionFromAcceleration(&self, finalvalue: f64, finalvelocity: f64, acceleration: f64) -> windows_core::Result<IUIAnimationTransition> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateParabolicTransitionFromAcceleration)(windows_core::Interface::as_raw(self), finalvalue, finalvelocity, acceleration, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUIAnimationTransitionLibrary_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateInstantaneousTransition: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateConstantTransition: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDiscreteTransition: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateLinearTransition: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateLinearTransitionFromSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSinusoidalTransitionFromVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSinusoidalTransitionFromRange: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64, f64, f64, UI_ANIMATION_SLOPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateAccelerateDecelerateTransition: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64, f64, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateReversalTransition: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateCubicTransition: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSmoothStopTransition: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateParabolicTransitionFromAcceleration: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIAnimationTransitionLibrary_Impl: Sized + windows_core::IUnknownImpl {
    fn CreateInstantaneousTransition(&self, finalvalue: f64) -> windows_core::Result<IUIAnimationTransition>;
    fn CreateConstantTransition(&self, duration: f64) -> windows_core::Result<IUIAnimationTransition>;
    fn CreateDiscreteTransition(&self, delay: f64, finalvalue: f64, hold: f64) -> windows_core::Result<IUIAnimationTransition>;
    fn CreateLinearTransition(&self, duration: f64, finalvalue: f64) -> windows_core::Result<IUIAnimationTransition>;
    fn CreateLinearTransitionFromSpeed(&self, speed: f64, finalvalue: f64) -> windows_core::Result<IUIAnimationTransition>;
    fn CreateSinusoidalTransitionFromVelocity(&self, duration: f64, period: f64) -> windows_core::Result<IUIAnimationTransition>;
    fn CreateSinusoidalTransitionFromRange(&self, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE) -> windows_core::Result<IUIAnimationTransition>;
    fn CreateAccelerateDecelerateTransition(&self, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64) -> windows_core::Result<IUIAnimationTransition>;
    fn CreateReversalTransition(&self, duration: f64) -> windows_core::Result<IUIAnimationTransition>;
    fn CreateCubicTransition(&self, duration: f64, finalvalue: f64, finalvelocity: f64) -> windows_core::Result<IUIAnimationTransition>;
    fn CreateSmoothStopTransition(&self, maximumduration: f64, finalvalue: f64) -> windows_core::Result<IUIAnimationTransition>;
    fn CreateParabolicTransitionFromAcceleration(&self, finalvalue: f64, finalvelocity: f64, acceleration: f64) -> windows_core::Result<IUIAnimationTransition>;
}
impl windows_core::RuntimeName for IUIAnimationTransitionLibrary {}
impl IUIAnimationTransitionLibrary_Vtbl {
    pub const fn new<Identity: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>() -> IUIAnimationTransitionLibrary_Vtbl {
        unsafe extern "system" fn CreateInstantaneousTransition<Identity: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finalvalue: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary_Impl::CreateInstantaneousTransition(this, core::mem::transmute_copy(&finalvalue)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateConstantTransition<Identity: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary_Impl::CreateConstantTransition(this, core::mem::transmute_copy(&duration)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDiscreteTransition<Identity: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: f64, finalvalue: f64, hold: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary_Impl::CreateDiscreteTransition(this, core::mem::transmute_copy(&delay), core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&hold)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearTransition<Identity: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, finalvalue: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary_Impl::CreateLinearTransition(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&finalvalue)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearTransitionFromSpeed<Identity: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, speed: f64, finalvalue: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary_Impl::CreateLinearTransitionFromSpeed(this, core::mem::transmute_copy(&speed), core::mem::transmute_copy(&finalvalue)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSinusoidalTransitionFromVelocity<Identity: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, period: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary_Impl::CreateSinusoidalTransitionFromVelocity(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&period)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSinusoidalTransitionFromRange<Identity: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary_Impl::CreateSinusoidalTransitionFromRange(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&minimumvalue), core::mem::transmute_copy(&maximumvalue), core::mem::transmute_copy(&period), core::mem::transmute_copy(&slope)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAccelerateDecelerateTransition<Identity: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary_Impl::CreateAccelerateDecelerateTransition(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&accelerationratio), core::mem::transmute_copy(&decelerationratio)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateReversalTransition<Identity: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary_Impl::CreateReversalTransition(this, core::mem::transmute_copy(&duration)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCubicTransition<Identity: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, finalvalue: f64, finalvelocity: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary_Impl::CreateCubicTransition(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&finalvelocity)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSmoothStopTransition<Identity: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maximumduration: f64, finalvalue: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary_Impl::CreateSmoothStopTransition(this, core::mem::transmute_copy(&maximumduration), core::mem::transmute_copy(&finalvalue)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateParabolicTransitionFromAcceleration<Identity: IUIAnimationTransitionLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finalvalue: f64, finalvelocity: f64, acceleration: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary_Impl::CreateParabolicTransitionFromAcceleration(this, core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&finalvelocity), core::mem::transmute_copy(&acceleration)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateInstantaneousTransition: CreateInstantaneousTransition::<Identity, OFFSET>,
            CreateConstantTransition: CreateConstantTransition::<Identity, OFFSET>,
            CreateDiscreteTransition: CreateDiscreteTransition::<Identity, OFFSET>,
            CreateLinearTransition: CreateLinearTransition::<Identity, OFFSET>,
            CreateLinearTransitionFromSpeed: CreateLinearTransitionFromSpeed::<Identity, OFFSET>,
            CreateSinusoidalTransitionFromVelocity: CreateSinusoidalTransitionFromVelocity::<Identity, OFFSET>,
            CreateSinusoidalTransitionFromRange: CreateSinusoidalTransitionFromRange::<Identity, OFFSET>,
            CreateAccelerateDecelerateTransition: CreateAccelerateDecelerateTransition::<Identity, OFFSET>,
            CreateReversalTransition: CreateReversalTransition::<Identity, OFFSET>,
            CreateCubicTransition: CreateCubicTransition::<Identity, OFFSET>,
            CreateSmoothStopTransition: CreateSmoothStopTransition::<Identity, OFFSET>,
            CreateParabolicTransitionFromAcceleration: CreateParabolicTransitionFromAcceleration::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationTransitionLibrary as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIAnimationTransitionLibrary2, IUIAnimationTransitionLibrary2_Vtbl, 0x03cfae53_9580_4ee3_b363_2ece51b4af6a);
impl core::ops::Deref for IUIAnimationTransitionLibrary2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAnimationTransitionLibrary2, windows_core::IUnknown);
impl IUIAnimationTransitionLibrary2 {
    pub unsafe fn CreateInstantaneousTransition(&self, finalvalue: f64) -> windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateInstantaneousTransition)(windows_core::Interface::as_raw(self), finalvalue, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateInstantaneousVectorTransition(&self, finalvalue: &[f64]) -> windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateInstantaneousVectorTransition)(windows_core::Interface::as_raw(self), core::mem::transmute(finalvalue.as_ptr()), finalvalue.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateConstantTransition(&self, duration: f64) -> windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateConstantTransition)(windows_core::Interface::as_raw(self), duration, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateDiscreteTransition(&self, delay: f64, finalvalue: f64, hold: f64) -> windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDiscreteTransition)(windows_core::Interface::as_raw(self), delay, finalvalue, hold, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateDiscreteVectorTransition(&self, delay: f64, finalvalue: &[f64], hold: f64) -> windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDiscreteVectorTransition)(windows_core::Interface::as_raw(self), delay, core::mem::transmute(finalvalue.as_ptr()), finalvalue.len().try_into().unwrap(), hold, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateLinearTransition(&self, duration: f64, finalvalue: f64) -> windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateLinearTransition)(windows_core::Interface::as_raw(self), duration, finalvalue, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateLinearVectorTransition(&self, duration: f64, finalvalue: &[f64]) -> windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateLinearVectorTransition)(windows_core::Interface::as_raw(self), duration, core::mem::transmute(finalvalue.as_ptr()), finalvalue.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateLinearTransitionFromSpeed(&self, speed: f64, finalvalue: f64) -> windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateLinearTransitionFromSpeed)(windows_core::Interface::as_raw(self), speed, finalvalue, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateLinearVectorTransitionFromSpeed(&self, speed: f64, finalvalue: &[f64]) -> windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateLinearVectorTransitionFromSpeed)(windows_core::Interface::as_raw(self), speed, core::mem::transmute(finalvalue.as_ptr()), finalvalue.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateSinusoidalTransitionFromVelocity(&self, duration: f64, period: f64) -> windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSinusoidalTransitionFromVelocity)(windows_core::Interface::as_raw(self), duration, period, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateSinusoidalTransitionFromRange(&self, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE) -> windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSinusoidalTransitionFromRange)(windows_core::Interface::as_raw(self), duration, minimumvalue, maximumvalue, period, slope, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateAccelerateDecelerateTransition(&self, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64) -> windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateAccelerateDecelerateTransition)(windows_core::Interface::as_raw(self), duration, finalvalue, accelerationratio, decelerationratio, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateReversalTransition(&self, duration: f64) -> windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateReversalTransition)(windows_core::Interface::as_raw(self), duration, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateCubicTransition(&self, duration: f64, finalvalue: f64, finalvelocity: f64) -> windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateCubicTransition)(windows_core::Interface::as_raw(self), duration, finalvalue, finalvelocity, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateCubicVectorTransition(&self, duration: f64, finalvalue: *const f64, finalvelocity: *const f64, cdimension: u32) -> windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateCubicVectorTransition)(windows_core::Interface::as_raw(self), duration, finalvalue, finalvelocity, cdimension, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateSmoothStopTransition(&self, maximumduration: f64, finalvalue: f64) -> windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSmoothStopTransition)(windows_core::Interface::as_raw(self), maximumduration, finalvalue, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateParabolicTransitionFromAcceleration(&self, finalvalue: f64, finalvelocity: f64, acceleration: f64) -> windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateParabolicTransitionFromAcceleration)(windows_core::Interface::as_raw(self), finalvalue, finalvelocity, acceleration, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateCubicBezierLinearTransition(&self, duration: f64, finalvalue: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateCubicBezierLinearTransition)(windows_core::Interface::as_raw(self), duration, finalvalue, x1, y1, x2, y2, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateCubicBezierLinearVectorTransition(&self, duration: f64, finalvalue: &[f64], x1: f64, y1: f64, x2: f64, y2: f64) -> windows_core::Result<IUIAnimationTransition2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateCubicBezierLinearVectorTransition)(windows_core::Interface::as_raw(self), duration, core::mem::transmute(finalvalue.as_ptr()), finalvalue.len().try_into().unwrap(), x1, y1, x2, y2, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUIAnimationTransitionLibrary2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateInstantaneousTransition: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInstantaneousVectorTransition: unsafe extern "system" fn(*mut core::ffi::c_void, *const f64, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateConstantTransition: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDiscreteTransition: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDiscreteVectorTransition: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *const f64, u32, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateLinearTransition: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateLinearVectorTransition: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *const f64, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateLinearTransitionFromSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateLinearVectorTransitionFromSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *const f64, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSinusoidalTransitionFromVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSinusoidalTransitionFromRange: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64, f64, f64, UI_ANIMATION_SLOPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateAccelerateDecelerateTransition: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64, f64, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateReversalTransition: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateCubicTransition: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateCubicVectorTransition: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *const f64, *const f64, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSmoothStopTransition: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateParabolicTransitionFromAcceleration: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateCubicBezierLinearTransition: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64, f64, f64, f64, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateCubicBezierLinearVectorTransition: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *const f64, u32, f64, f64, f64, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIAnimationTransitionLibrary2_Impl: Sized + windows_core::IUnknownImpl {
    fn CreateInstantaneousTransition(&self, finalvalue: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateInstantaneousVectorTransition(&self, finalvalue: *const f64, cdimension: u32) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateConstantTransition(&self, duration: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateDiscreteTransition(&self, delay: f64, finalvalue: f64, hold: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateDiscreteVectorTransition(&self, delay: f64, finalvalue: *const f64, cdimension: u32, hold: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateLinearTransition(&self, duration: f64, finalvalue: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateLinearVectorTransition(&self, duration: f64, finalvalue: *const f64, cdimension: u32) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateLinearTransitionFromSpeed(&self, speed: f64, finalvalue: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateLinearVectorTransitionFromSpeed(&self, speed: f64, finalvalue: *const f64, cdimension: u32) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateSinusoidalTransitionFromVelocity(&self, duration: f64, period: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateSinusoidalTransitionFromRange(&self, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateAccelerateDecelerateTransition(&self, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateReversalTransition(&self, duration: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateCubicTransition(&self, duration: f64, finalvalue: f64, finalvelocity: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateCubicVectorTransition(&self, duration: f64, finalvalue: *const f64, finalvelocity: *const f64, cdimension: u32) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateSmoothStopTransition(&self, maximumduration: f64, finalvalue: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateParabolicTransitionFromAcceleration(&self, finalvalue: f64, finalvelocity: f64, acceleration: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateCubicBezierLinearTransition(&self, duration: f64, finalvalue: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> windows_core::Result<IUIAnimationTransition2>;
    fn CreateCubicBezierLinearVectorTransition(&self, duration: f64, finalvalue: *const f64, cdimension: u32, x1: f64, y1: f64, x2: f64, y2: f64) -> windows_core::Result<IUIAnimationTransition2>;
}
impl windows_core::RuntimeName for IUIAnimationTransitionLibrary2 {}
impl IUIAnimationTransitionLibrary2_Vtbl {
    pub const fn new<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>() -> IUIAnimationTransitionLibrary2_Vtbl {
        unsafe extern "system" fn CreateInstantaneousTransition<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finalvalue: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateInstantaneousTransition(this, core::mem::transmute_copy(&finalvalue)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstantaneousVectorTransition<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finalvalue: *const f64, cdimension: u32, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateInstantaneousVectorTransition(this, core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&cdimension)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateConstantTransition<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateConstantTransition(this, core::mem::transmute_copy(&duration)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDiscreteTransition<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: f64, finalvalue: f64, hold: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateDiscreteTransition(this, core::mem::transmute_copy(&delay), core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&hold)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDiscreteVectorTransition<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: f64, finalvalue: *const f64, cdimension: u32, hold: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateDiscreteVectorTransition(this, core::mem::transmute_copy(&delay), core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&cdimension), core::mem::transmute_copy(&hold)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearTransition<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, finalvalue: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateLinearTransition(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&finalvalue)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearVectorTransition<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, finalvalue: *const f64, cdimension: u32, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateLinearVectorTransition(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&cdimension)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearTransitionFromSpeed<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, speed: f64, finalvalue: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateLinearTransitionFromSpeed(this, core::mem::transmute_copy(&speed), core::mem::transmute_copy(&finalvalue)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLinearVectorTransitionFromSpeed<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, speed: f64, finalvalue: *const f64, cdimension: u32, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateLinearVectorTransitionFromSpeed(this, core::mem::transmute_copy(&speed), core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&cdimension)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSinusoidalTransitionFromVelocity<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, period: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateSinusoidalTransitionFromVelocity(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&period)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSinusoidalTransitionFromRange<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, minimumvalue: f64, maximumvalue: f64, period: f64, slope: UI_ANIMATION_SLOPE, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateSinusoidalTransitionFromRange(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&minimumvalue), core::mem::transmute_copy(&maximumvalue), core::mem::transmute_copy(&period), core::mem::transmute_copy(&slope)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAccelerateDecelerateTransition<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, finalvalue: f64, accelerationratio: f64, decelerationratio: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateAccelerateDecelerateTransition(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&accelerationratio), core::mem::transmute_copy(&decelerationratio)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateReversalTransition<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateReversalTransition(this, core::mem::transmute_copy(&duration)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCubicTransition<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, finalvalue: f64, finalvelocity: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateCubicTransition(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&finalvelocity)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCubicVectorTransition<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, finalvalue: *const f64, finalvelocity: *const f64, cdimension: u32, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateCubicVectorTransition(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&finalvelocity), core::mem::transmute_copy(&cdimension)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSmoothStopTransition<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maximumduration: f64, finalvalue: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateSmoothStopTransition(this, core::mem::transmute_copy(&maximumduration), core::mem::transmute_copy(&finalvalue)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateParabolicTransitionFromAcceleration<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finalvalue: f64, finalvelocity: f64, acceleration: f64, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateParabolicTransitionFromAcceleration(this, core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&finalvelocity), core::mem::transmute_copy(&acceleration)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCubicBezierLinearTransition<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, finalvalue: f64, x1: f64, y1: f64, x2: f64, y2: f64, pptransition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateCubicBezierLinearTransition(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&x1), core::mem::transmute_copy(&y1), core::mem::transmute_copy(&x2), core::mem::transmute_copy(&y2)) {
                Ok(ok__) => {
                    pptransition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCubicBezierLinearVectorTransition<Identity: IUIAnimationTransitionLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64, finalvalue: *const f64, cdimension: u32, x1: f64, y1: f64, x2: f64, y2: f64, pptransition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationTransitionLibrary2_Impl::CreateCubicBezierLinearVectorTransition(this, core::mem::transmute_copy(&duration), core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&cdimension), core::mem::transmute_copy(&x1), core::mem::transmute_copy(&y1), core::mem::transmute_copy(&x2), core::mem::transmute_copy(&y2)) {
                Ok(ok__) => {
                    pptransition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateInstantaneousTransition: CreateInstantaneousTransition::<Identity, OFFSET>,
            CreateInstantaneousVectorTransition: CreateInstantaneousVectorTransition::<Identity, OFFSET>,
            CreateConstantTransition: CreateConstantTransition::<Identity, OFFSET>,
            CreateDiscreteTransition: CreateDiscreteTransition::<Identity, OFFSET>,
            CreateDiscreteVectorTransition: CreateDiscreteVectorTransition::<Identity, OFFSET>,
            CreateLinearTransition: CreateLinearTransition::<Identity, OFFSET>,
            CreateLinearVectorTransition: CreateLinearVectorTransition::<Identity, OFFSET>,
            CreateLinearTransitionFromSpeed: CreateLinearTransitionFromSpeed::<Identity, OFFSET>,
            CreateLinearVectorTransitionFromSpeed: CreateLinearVectorTransitionFromSpeed::<Identity, OFFSET>,
            CreateSinusoidalTransitionFromVelocity: CreateSinusoidalTransitionFromVelocity::<Identity, OFFSET>,
            CreateSinusoidalTransitionFromRange: CreateSinusoidalTransitionFromRange::<Identity, OFFSET>,
            CreateAccelerateDecelerateTransition: CreateAccelerateDecelerateTransition::<Identity, OFFSET>,
            CreateReversalTransition: CreateReversalTransition::<Identity, OFFSET>,
            CreateCubicTransition: CreateCubicTransition::<Identity, OFFSET>,
            CreateCubicVectorTransition: CreateCubicVectorTransition::<Identity, OFFSET>,
            CreateSmoothStopTransition: CreateSmoothStopTransition::<Identity, OFFSET>,
            CreateParabolicTransitionFromAcceleration: CreateParabolicTransitionFromAcceleration::<Identity, OFFSET>,
            CreateCubicBezierLinearTransition: CreateCubicBezierLinearTransition::<Identity, OFFSET>,
            CreateCubicBezierLinearVectorTransition: CreateCubicBezierLinearVectorTransition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationTransitionLibrary2 as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIAnimationVariable, IUIAnimationVariable_Vtbl, 0x8ceeb155_2849_4ce5_9448_91ff70e1e4d9);
impl core::ops::Deref for IUIAnimationVariable {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAnimationVariable, windows_core::IUnknown);
impl IUIAnimationVariable {
    pub unsafe fn GetValue(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetFinalValue(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFinalValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPreviousValue(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPreviousValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetIntegerValue(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIntegerValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetFinalIntegerValue(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFinalIntegerValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPreviousIntegerValue(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPreviousIntegerValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCurrentStoryboard(&self) -> windows_core::Result<IUIAnimationStoryboard> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentStoryboard)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLowerBound(&self, bound: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLowerBound)(windows_core::Interface::as_raw(self), bound).ok()
    }
    pub unsafe fn SetUpperBound(&self, bound: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetUpperBound)(windows_core::Interface::as_raw(self), bound).ok()
    }
    pub unsafe fn SetRoundingMode(&self, mode: UI_ANIMATION_ROUNDING_MODE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRoundingMode)(windows_core::Interface::as_raw(self), mode).ok()
    }
    pub unsafe fn SetTag<P0>(&self, object: P0, id: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetTag)(windows_core::Interface::as_raw(self), object.param().abi(), id).ok()
    }
    pub unsafe fn GetTag(&self, object: Option<*mut Option<windows_core::IUnknown>>, id: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTag)(windows_core::Interface::as_raw(self), core::mem::transmute(object.unwrap_or(core::ptr::null_mut())), core::mem::transmute(id.unwrap_or(core::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetVariableChangeHandler<P0>(&self, handler: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationVariableChangeHandler>,
    {
        (windows_core::Interface::vtable(self).SetVariableChangeHandler)(windows_core::Interface::as_raw(self), handler.param().abi()).ok()
    }
    pub unsafe fn SetVariableIntegerChangeHandler<P0>(&self, handler: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationVariableIntegerChangeHandler>,
    {
        (windows_core::Interface::vtable(self).SetVariableIntegerChangeHandler)(windows_core::Interface::as_raw(self), handler.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IUIAnimationVariable_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub GetFinalValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub GetPreviousValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub GetIntegerValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetFinalIntegerValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetPreviousIntegerValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetCurrentStoryboard: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLowerBound: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub SetUpperBound: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub SetRoundingMode: unsafe extern "system" fn(*mut core::ffi::c_void, UI_ANIMATION_ROUNDING_MODE) -> windows_core::HRESULT,
    pub SetTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetVariableChangeHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetVariableIntegerChangeHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIAnimationVariable_Impl: Sized + windows_core::IUnknownImpl {
    fn GetValue(&self) -> windows_core::Result<f64>;
    fn GetFinalValue(&self) -> windows_core::Result<f64>;
    fn GetPreviousValue(&self) -> windows_core::Result<f64>;
    fn GetIntegerValue(&self) -> windows_core::Result<i32>;
    fn GetFinalIntegerValue(&self) -> windows_core::Result<i32>;
    fn GetPreviousIntegerValue(&self) -> windows_core::Result<i32>;
    fn GetCurrentStoryboard(&self) -> windows_core::Result<IUIAnimationStoryboard>;
    fn SetLowerBound(&self, bound: f64) -> windows_core::Result<()>;
    fn SetUpperBound(&self, bound: f64) -> windows_core::Result<()>;
    fn SetRoundingMode(&self, mode: UI_ANIMATION_ROUNDING_MODE) -> windows_core::Result<()>;
    fn SetTag(&self, object: Option<&windows_core::IUnknown>, id: u32) -> windows_core::Result<()>;
    fn GetTag(&self, object: *mut Option<windows_core::IUnknown>, id: *mut u32) -> windows_core::Result<()>;
    fn SetVariableChangeHandler(&self, handler: Option<&IUIAnimationVariableChangeHandler>) -> windows_core::Result<()>;
    fn SetVariableIntegerChangeHandler(&self, handler: Option<&IUIAnimationVariableIntegerChangeHandler>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationVariable {}
impl IUIAnimationVariable_Vtbl {
    pub const fn new<Identity: IUIAnimationVariable_Impl, const OFFSET: isize>() -> IUIAnimationVariable_Vtbl {
        unsafe extern "system" fn GetValue<Identity: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationVariable_Impl::GetValue(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalValue<Identity: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finalvalue: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationVariable_Impl::GetFinalValue(this) {
                Ok(ok__) => {
                    finalvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviousValue<Identity: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, previousvalue: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationVariable_Impl::GetPreviousValue(this) {
                Ok(ok__) => {
                    previousvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIntegerValue<Identity: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationVariable_Impl::GetIntegerValue(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalIntegerValue<Identity: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finalvalue: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationVariable_Impl::GetFinalIntegerValue(this) {
                Ok(ok__) => {
                    finalvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviousIntegerValue<Identity: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, previousvalue: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationVariable_Impl::GetPreviousIntegerValue(this) {
                Ok(ok__) => {
                    previousvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentStoryboard<Identity: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationVariable_Impl::GetCurrentStoryboard(this) {
                Ok(ok__) => {
                    storyboard.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLowerBound<Identity: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bound: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable_Impl::SetLowerBound(this, core::mem::transmute_copy(&bound)).into()
        }
        unsafe extern "system" fn SetUpperBound<Identity: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bound: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable_Impl::SetUpperBound(this, core::mem::transmute_copy(&bound)).into()
        }
        unsafe extern "system" fn SetRoundingMode<Identity: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: UI_ANIMATION_ROUNDING_MODE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable_Impl::SetRoundingMode(this, core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn SetTag<Identity: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, id: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable_Impl::SetTag(this, windows_core::from_raw_borrowed(&object), core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetTag<Identity: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut *mut core::ffi::c_void, id: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable_Impl::GetTag(this, core::mem::transmute_copy(&object), core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn SetVariableChangeHandler<Identity: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable_Impl::SetVariableChangeHandler(this, windows_core::from_raw_borrowed(&handler)).into()
        }
        unsafe extern "system" fn SetVariableIntegerChangeHandler<Identity: IUIAnimationVariable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable_Impl::SetVariableIntegerChangeHandler(this, windows_core::from_raw_borrowed(&handler)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetValue: GetValue::<Identity, OFFSET>,
            GetFinalValue: GetFinalValue::<Identity, OFFSET>,
            GetPreviousValue: GetPreviousValue::<Identity, OFFSET>,
            GetIntegerValue: GetIntegerValue::<Identity, OFFSET>,
            GetFinalIntegerValue: GetFinalIntegerValue::<Identity, OFFSET>,
            GetPreviousIntegerValue: GetPreviousIntegerValue::<Identity, OFFSET>,
            GetCurrentStoryboard: GetCurrentStoryboard::<Identity, OFFSET>,
            SetLowerBound: SetLowerBound::<Identity, OFFSET>,
            SetUpperBound: SetUpperBound::<Identity, OFFSET>,
            SetRoundingMode: SetRoundingMode::<Identity, OFFSET>,
            SetTag: SetTag::<Identity, OFFSET>,
            GetTag: GetTag::<Identity, OFFSET>,
            SetVariableChangeHandler: SetVariableChangeHandler::<Identity, OFFSET>,
            SetVariableIntegerChangeHandler: SetVariableIntegerChangeHandler::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationVariable as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIAnimationVariable2, IUIAnimationVariable2_Vtbl, 0x4914b304_96ab_44d9_9e77_d5109b7e7466);
impl core::ops::Deref for IUIAnimationVariable2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAnimationVariable2, windows_core::IUnknown);
impl IUIAnimationVariable2 {
    pub unsafe fn GetDimension(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDimension)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetValue(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetVectorValue(&self, value: &mut [f64]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetVectorValue)(windows_core::Interface::as_raw(self), core::mem::transmute(value.as_ptr()), value.len().try_into().unwrap()).ok()
    }
    #[cfg(feature = "Win32_Graphics_DirectComposition")]
    pub unsafe fn GetCurve<P0>(&self, animation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::DirectComposition::IDCompositionAnimation>,
    {
        (windows_core::Interface::vtable(self).GetCurve)(windows_core::Interface::as_raw(self), animation.param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_DirectComposition")]
    pub unsafe fn GetVectorCurve(&self, animation: &[Option<super::super::Graphics::DirectComposition::IDCompositionAnimation>]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetVectorCurve)(windows_core::Interface::as_raw(self), core::mem::transmute(animation.as_ptr()), animation.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetFinalValue(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFinalValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetFinalVectorValue(&self, finalvalue: &mut [f64]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFinalVectorValue)(windows_core::Interface::as_raw(self), core::mem::transmute(finalvalue.as_ptr()), finalvalue.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetPreviousValue(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPreviousValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPreviousVectorValue(&self, previousvalue: &mut [f64]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPreviousVectorValue)(windows_core::Interface::as_raw(self), core::mem::transmute(previousvalue.as_ptr()), previousvalue.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetIntegerValue(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIntegerValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetIntegerVectorValue(&self, value: &mut [i32]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetIntegerVectorValue)(windows_core::Interface::as_raw(self), core::mem::transmute(value.as_ptr()), value.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetFinalIntegerValue(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFinalIntegerValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetFinalIntegerVectorValue(&self, finalvalue: &mut [i32]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFinalIntegerVectorValue)(windows_core::Interface::as_raw(self), core::mem::transmute(finalvalue.as_ptr()), finalvalue.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetPreviousIntegerValue(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPreviousIntegerValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPreviousIntegerVectorValue(&self, previousvalue: &mut [i32]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPreviousIntegerVectorValue)(windows_core::Interface::as_raw(self), core::mem::transmute(previousvalue.as_ptr()), previousvalue.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetCurrentStoryboard(&self) -> windows_core::Result<IUIAnimationStoryboard2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentStoryboard)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLowerBound(&self, bound: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLowerBound)(windows_core::Interface::as_raw(self), bound).ok()
    }
    pub unsafe fn SetLowerBoundVector(&self, bound: &[f64]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLowerBoundVector)(windows_core::Interface::as_raw(self), core::mem::transmute(bound.as_ptr()), bound.len().try_into().unwrap()).ok()
    }
    pub unsafe fn SetUpperBound(&self, bound: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetUpperBound)(windows_core::Interface::as_raw(self), bound).ok()
    }
    pub unsafe fn SetUpperBoundVector(&self, bound: &[f64]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetUpperBoundVector)(windows_core::Interface::as_raw(self), core::mem::transmute(bound.as_ptr()), bound.len().try_into().unwrap()).ok()
    }
    pub unsafe fn SetRoundingMode(&self, mode: UI_ANIMATION_ROUNDING_MODE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRoundingMode)(windows_core::Interface::as_raw(self), mode).ok()
    }
    pub unsafe fn SetTag<P0>(&self, object: P0, id: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetTag)(windows_core::Interface::as_raw(self), object.param().abi(), id).ok()
    }
    pub unsafe fn GetTag(&self, object: Option<*mut Option<windows_core::IUnknown>>, id: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTag)(windows_core::Interface::as_raw(self), core::mem::transmute(object.unwrap_or(core::ptr::null_mut())), core::mem::transmute(id.unwrap_or(core::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetVariableChangeHandler<P0, P1>(&self, handler: P0, fregisterfornextanimationevent: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationVariableChangeHandler2>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetVariableChangeHandler)(windows_core::Interface::as_raw(self), handler.param().abi(), fregisterfornextanimationevent.param().abi()).ok()
    }
    pub unsafe fn SetVariableIntegerChangeHandler<P0, P1>(&self, handler: P0, fregisterfornextanimationevent: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationVariableIntegerChangeHandler2>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetVariableIntegerChangeHandler)(windows_core::Interface::as_raw(self), handler.param().abi(), fregisterfornextanimationevent.param().abi()).ok()
    }
    pub unsafe fn SetVariableCurveChangeHandler<P0>(&self, handler: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationVariableCurveChangeHandler2>,
    {
        (windows_core::Interface::vtable(self).SetVariableCurveChangeHandler)(windows_core::Interface::as_raw(self), handler.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IUIAnimationVariable2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDimension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub GetVectorValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_DirectComposition")]
    pub GetCurve: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_DirectComposition"))]
    GetCurve: usize,
    #[cfg(feature = "Win32_Graphics_DirectComposition")]
    pub GetVectorCurve: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_DirectComposition"))]
    GetVectorCurve: usize,
    pub GetFinalValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub GetFinalVectorValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64, u32) -> windows_core::HRESULT,
    pub GetPreviousValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub GetPreviousVectorValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64, u32) -> windows_core::HRESULT,
    pub GetIntegerValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetIntegerVectorValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, u32) -> windows_core::HRESULT,
    pub GetFinalIntegerValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetFinalIntegerVectorValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, u32) -> windows_core::HRESULT,
    pub GetPreviousIntegerValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetPreviousIntegerVectorValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, u32) -> windows_core::HRESULT,
    pub GetCurrentStoryboard: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLowerBound: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub SetLowerBoundVector: unsafe extern "system" fn(*mut core::ffi::c_void, *const f64, u32) -> windows_core::HRESULT,
    pub SetUpperBound: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub SetUpperBoundVector: unsafe extern "system" fn(*mut core::ffi::c_void, *const f64, u32) -> windows_core::HRESULT,
    pub SetRoundingMode: unsafe extern "system" fn(*mut core::ffi::c_void, UI_ANIMATION_ROUNDING_MODE) -> windows_core::HRESULT,
    pub SetTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetVariableChangeHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetVariableIntegerChangeHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetVariableCurveChangeHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Graphics_DirectComposition")]
pub trait IUIAnimationVariable2_Impl: Sized + windows_core::IUnknownImpl {
    fn GetDimension(&self) -> windows_core::Result<u32>;
    fn GetValue(&self) -> windows_core::Result<f64>;
    fn GetVectorValue(&self, value: *mut f64, cdimension: u32) -> windows_core::Result<()>;
    fn GetCurve(&self, animation: Option<&super::super::Graphics::DirectComposition::IDCompositionAnimation>) -> windows_core::Result<()>;
    fn GetVectorCurve(&self, animation: *const Option<super::super::Graphics::DirectComposition::IDCompositionAnimation>, cdimension: u32) -> windows_core::Result<()>;
    fn GetFinalValue(&self) -> windows_core::Result<f64>;
    fn GetFinalVectorValue(&self, finalvalue: *mut f64, cdimension: u32) -> windows_core::Result<()>;
    fn GetPreviousValue(&self) -> windows_core::Result<f64>;
    fn GetPreviousVectorValue(&self, previousvalue: *mut f64, cdimension: u32) -> windows_core::Result<()>;
    fn GetIntegerValue(&self) -> windows_core::Result<i32>;
    fn GetIntegerVectorValue(&self, value: *mut i32, cdimension: u32) -> windows_core::Result<()>;
    fn GetFinalIntegerValue(&self) -> windows_core::Result<i32>;
    fn GetFinalIntegerVectorValue(&self, finalvalue: *mut i32, cdimension: u32) -> windows_core::Result<()>;
    fn GetPreviousIntegerValue(&self) -> windows_core::Result<i32>;
    fn GetPreviousIntegerVectorValue(&self, previousvalue: *mut i32, cdimension: u32) -> windows_core::Result<()>;
    fn GetCurrentStoryboard(&self) -> windows_core::Result<IUIAnimationStoryboard2>;
    fn SetLowerBound(&self, bound: f64) -> windows_core::Result<()>;
    fn SetLowerBoundVector(&self, bound: *const f64, cdimension: u32) -> windows_core::Result<()>;
    fn SetUpperBound(&self, bound: f64) -> windows_core::Result<()>;
    fn SetUpperBoundVector(&self, bound: *const f64, cdimension: u32) -> windows_core::Result<()>;
    fn SetRoundingMode(&self, mode: UI_ANIMATION_ROUNDING_MODE) -> windows_core::Result<()>;
    fn SetTag(&self, object: Option<&windows_core::IUnknown>, id: u32) -> windows_core::Result<()>;
    fn GetTag(&self, object: *mut Option<windows_core::IUnknown>, id: *mut u32) -> windows_core::Result<()>;
    fn SetVariableChangeHandler(&self, handler: Option<&IUIAnimationVariableChangeHandler2>, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetVariableIntegerChangeHandler(&self, handler: Option<&IUIAnimationVariableIntegerChangeHandler2>, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetVariableCurveChangeHandler(&self, handler: Option<&IUIAnimationVariableCurveChangeHandler2>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_DirectComposition")]
impl windows_core::RuntimeName for IUIAnimationVariable2 {}
#[cfg(feature = "Win32_Graphics_DirectComposition")]
impl IUIAnimationVariable2_Vtbl {
    pub const fn new<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>() -> IUIAnimationVariable2_Vtbl {
        unsafe extern "system" fn GetDimension<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dimension: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationVariable2_Impl::GetDimension(this) {
                Ok(ok__) => {
                    dimension.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationVariable2_Impl::GetValue(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVectorValue<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut f64, cdimension: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable2_Impl::GetVectorValue(this, core::mem::transmute_copy(&value), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetCurve<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable2_Impl::GetCurve(this, windows_core::from_raw_borrowed(&animation)).into()
        }
        unsafe extern "system" fn GetVectorCurve<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, animation: *const *mut core::ffi::c_void, cdimension: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable2_Impl::GetVectorCurve(this, core::mem::transmute_copy(&animation), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetFinalValue<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finalvalue: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationVariable2_Impl::GetFinalValue(this) {
                Ok(ok__) => {
                    finalvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalVectorValue<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finalvalue: *mut f64, cdimension: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable2_Impl::GetFinalVectorValue(this, core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetPreviousValue<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, previousvalue: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationVariable2_Impl::GetPreviousValue(this) {
                Ok(ok__) => {
                    previousvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviousVectorValue<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, previousvalue: *mut f64, cdimension: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable2_Impl::GetPreviousVectorValue(this, core::mem::transmute_copy(&previousvalue), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetIntegerValue<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationVariable2_Impl::GetIntegerValue(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIntegerVectorValue<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32, cdimension: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable2_Impl::GetIntegerVectorValue(this, core::mem::transmute_copy(&value), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetFinalIntegerValue<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finalvalue: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationVariable2_Impl::GetFinalIntegerValue(this) {
                Ok(ok__) => {
                    finalvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFinalIntegerVectorValue<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finalvalue: *mut i32, cdimension: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable2_Impl::GetFinalIntegerVectorValue(this, core::mem::transmute_copy(&finalvalue), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetPreviousIntegerValue<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, previousvalue: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationVariable2_Impl::GetPreviousIntegerValue(this) {
                Ok(ok__) => {
                    previousvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviousIntegerVectorValue<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, previousvalue: *mut i32, cdimension: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable2_Impl::GetPreviousIntegerVectorValue(this, core::mem::transmute_copy(&previousvalue), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn GetCurrentStoryboard<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUIAnimationVariable2_Impl::GetCurrentStoryboard(this) {
                Ok(ok__) => {
                    storyboard.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLowerBound<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bound: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable2_Impl::SetLowerBound(this, core::mem::transmute_copy(&bound)).into()
        }
        unsafe extern "system" fn SetLowerBoundVector<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bound: *const f64, cdimension: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable2_Impl::SetLowerBoundVector(this, core::mem::transmute_copy(&bound), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn SetUpperBound<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bound: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable2_Impl::SetUpperBound(this, core::mem::transmute_copy(&bound)).into()
        }
        unsafe extern "system" fn SetUpperBoundVector<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bound: *const f64, cdimension: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable2_Impl::SetUpperBoundVector(this, core::mem::transmute_copy(&bound), core::mem::transmute_copy(&cdimension)).into()
        }
        unsafe extern "system" fn SetRoundingMode<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: UI_ANIMATION_ROUNDING_MODE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable2_Impl::SetRoundingMode(this, core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn SetTag<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, id: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable2_Impl::SetTag(this, windows_core::from_raw_borrowed(&object), core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetTag<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut *mut core::ffi::c_void, id: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable2_Impl::GetTag(this, core::mem::transmute_copy(&object), core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn SetVariableChangeHandler<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable2_Impl::SetVariableChangeHandler(this, windows_core::from_raw_borrowed(&handler), core::mem::transmute_copy(&fregisterfornextanimationevent)).into()
        }
        unsafe extern "system" fn SetVariableIntegerChangeHandler<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, fregisterfornextanimationevent: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable2_Impl::SetVariableIntegerChangeHandler(this, windows_core::from_raw_borrowed(&handler), core::mem::transmute_copy(&fregisterfornextanimationevent)).into()
        }
        unsafe extern "system" fn SetVariableCurveChangeHandler<Identity: IUIAnimationVariable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariable2_Impl::SetVariableCurveChangeHandler(this, windows_core::from_raw_borrowed(&handler)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDimension: GetDimension::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            GetVectorValue: GetVectorValue::<Identity, OFFSET>,
            GetCurve: GetCurve::<Identity, OFFSET>,
            GetVectorCurve: GetVectorCurve::<Identity, OFFSET>,
            GetFinalValue: GetFinalValue::<Identity, OFFSET>,
            GetFinalVectorValue: GetFinalVectorValue::<Identity, OFFSET>,
            GetPreviousValue: GetPreviousValue::<Identity, OFFSET>,
            GetPreviousVectorValue: GetPreviousVectorValue::<Identity, OFFSET>,
            GetIntegerValue: GetIntegerValue::<Identity, OFFSET>,
            GetIntegerVectorValue: GetIntegerVectorValue::<Identity, OFFSET>,
            GetFinalIntegerValue: GetFinalIntegerValue::<Identity, OFFSET>,
            GetFinalIntegerVectorValue: GetFinalIntegerVectorValue::<Identity, OFFSET>,
            GetPreviousIntegerValue: GetPreviousIntegerValue::<Identity, OFFSET>,
            GetPreviousIntegerVectorValue: GetPreviousIntegerVectorValue::<Identity, OFFSET>,
            GetCurrentStoryboard: GetCurrentStoryboard::<Identity, OFFSET>,
            SetLowerBound: SetLowerBound::<Identity, OFFSET>,
            SetLowerBoundVector: SetLowerBoundVector::<Identity, OFFSET>,
            SetUpperBound: SetUpperBound::<Identity, OFFSET>,
            SetUpperBoundVector: SetUpperBoundVector::<Identity, OFFSET>,
            SetRoundingMode: SetRoundingMode::<Identity, OFFSET>,
            SetTag: SetTag::<Identity, OFFSET>,
            GetTag: GetTag::<Identity, OFFSET>,
            SetVariableChangeHandler: SetVariableChangeHandler::<Identity, OFFSET>,
            SetVariableIntegerChangeHandler: SetVariableIntegerChangeHandler::<Identity, OFFSET>,
            SetVariableCurveChangeHandler: SetVariableCurveChangeHandler::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationVariable2 as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIAnimationVariableChangeHandler, IUIAnimationVariableChangeHandler_Vtbl, 0x6358b7ba_87d2_42d5_bf71_82e919dd5862);
impl core::ops::Deref for IUIAnimationVariableChangeHandler {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAnimationVariableChangeHandler, windows_core::IUnknown);
impl IUIAnimationVariableChangeHandler {
    pub unsafe fn OnValueChanged<P0, P1>(&self, storyboard: P0, variable: P1, newvalue: f64, previousvalue: f64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationStoryboard>,
        P1: windows_core::Param<IUIAnimationVariable>,
    {
        (windows_core::Interface::vtable(self).OnValueChanged)(windows_core::Interface::as_raw(self), storyboard.param().abi(), variable.param().abi(), newvalue, previousvalue).ok()
    }
}
#[repr(C)]
pub struct IUIAnimationVariableChangeHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnValueChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
}
pub trait IUIAnimationVariableChangeHandler_Impl: Sized + windows_core::IUnknownImpl {
    fn OnValueChanged(&self, storyboard: Option<&IUIAnimationStoryboard>, variable: Option<&IUIAnimationVariable>, newvalue: f64, previousvalue: f64) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationVariableChangeHandler {}
impl IUIAnimationVariableChangeHandler_Vtbl {
    pub const fn new<Identity: IUIAnimationVariableChangeHandler_Impl, const OFFSET: isize>() -> IUIAnimationVariableChangeHandler_Vtbl {
        unsafe extern "system" fn OnValueChanged<Identity: IUIAnimationVariableChangeHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut core::ffi::c_void, variable: *mut core::ffi::c_void, newvalue: f64, previousvalue: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariableChangeHandler_Impl::OnValueChanged(this, windows_core::from_raw_borrowed(&storyboard), windows_core::from_raw_borrowed(&variable), core::mem::transmute_copy(&newvalue), core::mem::transmute_copy(&previousvalue)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnValueChanged: OnValueChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationVariableChangeHandler as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIAnimationVariableChangeHandler2, IUIAnimationVariableChangeHandler2_Vtbl, 0x63acc8d2_6eae_4bb0_b879_586dd8cfbe42);
impl core::ops::Deref for IUIAnimationVariableChangeHandler2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAnimationVariableChangeHandler2, windows_core::IUnknown);
impl IUIAnimationVariableChangeHandler2 {
    pub unsafe fn OnValueChanged<P0, P1>(&self, storyboard: P0, variable: P1, newvalue: *const f64, previousvalue: *const f64, cdimension: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationStoryboard2>,
        P1: windows_core::Param<IUIAnimationVariable2>,
    {
        (windows_core::Interface::vtable(self).OnValueChanged)(windows_core::Interface::as_raw(self), storyboard.param().abi(), variable.param().abi(), newvalue, previousvalue, cdimension).ok()
    }
}
#[repr(C)]
pub struct IUIAnimationVariableChangeHandler2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnValueChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const f64, *const f64, u32) -> windows_core::HRESULT,
}
pub trait IUIAnimationVariableChangeHandler2_Impl: Sized + windows_core::IUnknownImpl {
    fn OnValueChanged(&self, storyboard: Option<&IUIAnimationStoryboard2>, variable: Option<&IUIAnimationVariable2>, newvalue: *const f64, previousvalue: *const f64, cdimension: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationVariableChangeHandler2 {}
impl IUIAnimationVariableChangeHandler2_Vtbl {
    pub const fn new<Identity: IUIAnimationVariableChangeHandler2_Impl, const OFFSET: isize>() -> IUIAnimationVariableChangeHandler2_Vtbl {
        unsafe extern "system" fn OnValueChanged<Identity: IUIAnimationVariableChangeHandler2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut core::ffi::c_void, variable: *mut core::ffi::c_void, newvalue: *const f64, previousvalue: *const f64, cdimension: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariableChangeHandler2_Impl::OnValueChanged(this, windows_core::from_raw_borrowed(&storyboard), windows_core::from_raw_borrowed(&variable), core::mem::transmute_copy(&newvalue), core::mem::transmute_copy(&previousvalue), core::mem::transmute_copy(&cdimension)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnValueChanged: OnValueChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationVariableChangeHandler2 as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIAnimationVariableCurveChangeHandler2, IUIAnimationVariableCurveChangeHandler2_Vtbl, 0x72895e91_0145_4c21_9192_5aab40eddf80);
impl core::ops::Deref for IUIAnimationVariableCurveChangeHandler2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAnimationVariableCurveChangeHandler2, windows_core::IUnknown);
impl IUIAnimationVariableCurveChangeHandler2 {
    pub unsafe fn OnCurveChanged<P0>(&self, variable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationVariable2>,
    {
        (windows_core::Interface::vtable(self).OnCurveChanged)(windows_core::Interface::as_raw(self), variable.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IUIAnimationVariableCurveChangeHandler2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnCurveChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIAnimationVariableCurveChangeHandler2_Impl: Sized + windows_core::IUnknownImpl {
    fn OnCurveChanged(&self, variable: Option<&IUIAnimationVariable2>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationVariableCurveChangeHandler2 {}
impl IUIAnimationVariableCurveChangeHandler2_Vtbl {
    pub const fn new<Identity: IUIAnimationVariableCurveChangeHandler2_Impl, const OFFSET: isize>() -> IUIAnimationVariableCurveChangeHandler2_Vtbl {
        unsafe extern "system" fn OnCurveChanged<Identity: IUIAnimationVariableCurveChangeHandler2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, variable: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariableCurveChangeHandler2_Impl::OnCurveChanged(this, windows_core::from_raw_borrowed(&variable)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnCurveChanged: OnCurveChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationVariableCurveChangeHandler2 as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIAnimationVariableIntegerChangeHandler, IUIAnimationVariableIntegerChangeHandler_Vtbl, 0xbb3e1550_356e_44b0_99da_85ac6017865e);
impl core::ops::Deref for IUIAnimationVariableIntegerChangeHandler {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAnimationVariableIntegerChangeHandler, windows_core::IUnknown);
impl IUIAnimationVariableIntegerChangeHandler {
    pub unsafe fn OnIntegerValueChanged<P0, P1>(&self, storyboard: P0, variable: P1, newvalue: i32, previousvalue: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationStoryboard>,
        P1: windows_core::Param<IUIAnimationVariable>,
    {
        (windows_core::Interface::vtable(self).OnIntegerValueChanged)(windows_core::Interface::as_raw(self), storyboard.param().abi(), variable.param().abi(), newvalue, previousvalue).ok()
    }
}
#[repr(C)]
pub struct IUIAnimationVariableIntegerChangeHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnIntegerValueChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
}
pub trait IUIAnimationVariableIntegerChangeHandler_Impl: Sized + windows_core::IUnknownImpl {
    fn OnIntegerValueChanged(&self, storyboard: Option<&IUIAnimationStoryboard>, variable: Option<&IUIAnimationVariable>, newvalue: i32, previousvalue: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationVariableIntegerChangeHandler {}
impl IUIAnimationVariableIntegerChangeHandler_Vtbl {
    pub const fn new<Identity: IUIAnimationVariableIntegerChangeHandler_Impl, const OFFSET: isize>() -> IUIAnimationVariableIntegerChangeHandler_Vtbl {
        unsafe extern "system" fn OnIntegerValueChanged<Identity: IUIAnimationVariableIntegerChangeHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut core::ffi::c_void, variable: *mut core::ffi::c_void, newvalue: i32, previousvalue: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariableIntegerChangeHandler_Impl::OnIntegerValueChanged(this, windows_core::from_raw_borrowed(&storyboard), windows_core::from_raw_borrowed(&variable), core::mem::transmute_copy(&newvalue), core::mem::transmute_copy(&previousvalue)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnIntegerValueChanged: OnIntegerValueChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationVariableIntegerChangeHandler as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IUIAnimationVariableIntegerChangeHandler2, IUIAnimationVariableIntegerChangeHandler2_Vtbl, 0x829b6cf1_4f3a_4412_ae09_b243eb4c6b58);
impl core::ops::Deref for IUIAnimationVariableIntegerChangeHandler2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAnimationVariableIntegerChangeHandler2, windows_core::IUnknown);
impl IUIAnimationVariableIntegerChangeHandler2 {
    pub unsafe fn OnIntegerValueChanged<P0, P1>(&self, storyboard: P0, variable: P1, newvalue: *const i32, previousvalue: *const i32, cdimension: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUIAnimationStoryboard2>,
        P1: windows_core::Param<IUIAnimationVariable2>,
    {
        (windows_core::Interface::vtable(self).OnIntegerValueChanged)(windows_core::Interface::as_raw(self), storyboard.param().abi(), variable.param().abi(), newvalue, previousvalue, cdimension).ok()
    }
}
#[repr(C)]
pub struct IUIAnimationVariableIntegerChangeHandler2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnIntegerValueChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const i32, *const i32, u32) -> windows_core::HRESULT,
}
pub trait IUIAnimationVariableIntegerChangeHandler2_Impl: Sized + windows_core::IUnknownImpl {
    fn OnIntegerValueChanged(&self, storyboard: Option<&IUIAnimationStoryboard2>, variable: Option<&IUIAnimationVariable2>, newvalue: *const i32, previousvalue: *const i32, cdimension: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUIAnimationVariableIntegerChangeHandler2 {}
impl IUIAnimationVariableIntegerChangeHandler2_Vtbl {
    pub const fn new<Identity: IUIAnimationVariableIntegerChangeHandler2_Impl, const OFFSET: isize>() -> IUIAnimationVariableIntegerChangeHandler2_Vtbl {
        unsafe extern "system" fn OnIntegerValueChanged<Identity: IUIAnimationVariableIntegerChangeHandler2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storyboard: *mut core::ffi::c_void, variable: *mut core::ffi::c_void, newvalue: *const i32, previousvalue: *const i32, cdimension: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUIAnimationVariableIntegerChangeHandler2_Impl::OnIntegerValueChanged(this, windows_core::from_raw_borrowed(&storyboard), windows_core::from_raw_borrowed(&variable), core::mem::transmute_copy(&newvalue), core::mem::transmute_copy(&previousvalue), core::mem::transmute_copy(&cdimension)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnIntegerValueChanged: OnIntegerValueChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAnimationVariableIntegerChangeHandler2 as windows_core::Interface>::IID
    }
}
pub const UI_ANIMATION_DEPENDENCY_DURATION: UI_ANIMATION_DEPENDENCIES = UI_ANIMATION_DEPENDENCIES(8i32);
pub const UI_ANIMATION_DEPENDENCY_FINAL_VALUE: UI_ANIMATION_DEPENDENCIES = UI_ANIMATION_DEPENDENCIES(2i32);
pub const UI_ANIMATION_DEPENDENCY_FINAL_VELOCITY: UI_ANIMATION_DEPENDENCIES = UI_ANIMATION_DEPENDENCIES(4i32);
pub const UI_ANIMATION_DEPENDENCY_INTERMEDIATE_VALUES: UI_ANIMATION_DEPENDENCIES = UI_ANIMATION_DEPENDENCIES(1i32);
pub const UI_ANIMATION_DEPENDENCY_NONE: UI_ANIMATION_DEPENDENCIES = UI_ANIMATION_DEPENDENCIES(0i32);
pub const UI_ANIMATION_IDLE_BEHAVIOR_CONTINUE: UI_ANIMATION_IDLE_BEHAVIOR = UI_ANIMATION_IDLE_BEHAVIOR(0i32);
pub const UI_ANIMATION_IDLE_BEHAVIOR_DISABLE: UI_ANIMATION_IDLE_BEHAVIOR = UI_ANIMATION_IDLE_BEHAVIOR(1i32);
pub const UI_ANIMATION_MANAGER_BUSY: UI_ANIMATION_MANAGER_STATUS = UI_ANIMATION_MANAGER_STATUS(1i32);
pub const UI_ANIMATION_MANAGER_IDLE: UI_ANIMATION_MANAGER_STATUS = UI_ANIMATION_MANAGER_STATUS(0i32);
pub const UI_ANIMATION_MODE_DISABLED: UI_ANIMATION_MODE = UI_ANIMATION_MODE(0i32);
pub const UI_ANIMATION_MODE_ENABLED: UI_ANIMATION_MODE = UI_ANIMATION_MODE(2i32);
pub const UI_ANIMATION_MODE_SYSTEM_DEFAULT: UI_ANIMATION_MODE = UI_ANIMATION_MODE(1i32);
pub const UI_ANIMATION_PRIORITY_EFFECT_DELAY: UI_ANIMATION_PRIORITY_EFFECT = UI_ANIMATION_PRIORITY_EFFECT(1i32);
pub const UI_ANIMATION_PRIORITY_EFFECT_FAILURE: UI_ANIMATION_PRIORITY_EFFECT = UI_ANIMATION_PRIORITY_EFFECT(0i32);
pub const UI_ANIMATION_REPEAT_INDEFINITELY: i32 = -1i32;
pub const UI_ANIMATION_REPEAT_INDEFINITELY_CONCLUDE_AT_END: i32 = -1i32;
pub const UI_ANIMATION_REPEAT_INDEFINITELY_CONCLUDE_AT_START: i32 = -2i32;
pub const UI_ANIMATION_REPEAT_MODE_ALTERNATE: UI_ANIMATION_REPEAT_MODE = UI_ANIMATION_REPEAT_MODE(1i32);
pub const UI_ANIMATION_REPEAT_MODE_NORMAL: UI_ANIMATION_REPEAT_MODE = UI_ANIMATION_REPEAT_MODE(0i32);
pub const UI_ANIMATION_ROUNDING_CEILING: UI_ANIMATION_ROUNDING_MODE = UI_ANIMATION_ROUNDING_MODE(2i32);
pub const UI_ANIMATION_ROUNDING_FLOOR: UI_ANIMATION_ROUNDING_MODE = UI_ANIMATION_ROUNDING_MODE(1i32);
pub const UI_ANIMATION_ROUNDING_NEAREST: UI_ANIMATION_ROUNDING_MODE = UI_ANIMATION_ROUNDING_MODE(0i32);
pub const UI_ANIMATION_SCHEDULING_ALREADY_SCHEDULED: UI_ANIMATION_SCHEDULING_RESULT = UI_ANIMATION_SCHEDULING_RESULT(2i32);
pub const UI_ANIMATION_SCHEDULING_DEFERRED: UI_ANIMATION_SCHEDULING_RESULT = UI_ANIMATION_SCHEDULING_RESULT(4i32);
pub const UI_ANIMATION_SCHEDULING_INSUFFICIENT_PRIORITY: UI_ANIMATION_SCHEDULING_RESULT = UI_ANIMATION_SCHEDULING_RESULT(1i32);
pub const UI_ANIMATION_SCHEDULING_SUCCEEDED: UI_ANIMATION_SCHEDULING_RESULT = UI_ANIMATION_SCHEDULING_RESULT(3i32);
pub const UI_ANIMATION_SCHEDULING_UNEXPECTED_FAILURE: UI_ANIMATION_SCHEDULING_RESULT = UI_ANIMATION_SCHEDULING_RESULT(0i32);
pub const UI_ANIMATION_SECONDS_EVENTUALLY: i32 = -1i32;
pub const UI_ANIMATION_SECONDS_INFINITE: i32 = -1i32;
pub const UI_ANIMATION_SLOPE_DECREASING: UI_ANIMATION_SLOPE = UI_ANIMATION_SLOPE(1i32);
pub const UI_ANIMATION_SLOPE_INCREASING: UI_ANIMATION_SLOPE = UI_ANIMATION_SLOPE(0i32);
pub const UI_ANIMATION_STORYBOARD_BUILDING: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(0i32);
pub const UI_ANIMATION_STORYBOARD_CANCELLED: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(2i32);
pub const UI_ANIMATION_STORYBOARD_FINISHED: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(5i32);
pub const UI_ANIMATION_STORYBOARD_INSUFFICIENT_PRIORITY: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(7i32);
pub const UI_ANIMATION_STORYBOARD_PLAYING: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(3i32);
pub const UI_ANIMATION_STORYBOARD_READY: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(6i32);
pub const UI_ANIMATION_STORYBOARD_SCHEDULED: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(1i32);
pub const UI_ANIMATION_STORYBOARD_TRUNCATED: UI_ANIMATION_STORYBOARD_STATUS = UI_ANIMATION_STORYBOARD_STATUS(4i32);
pub const UI_ANIMATION_TIMER_CLIENT_BUSY: UI_ANIMATION_TIMER_CLIENT_STATUS = UI_ANIMATION_TIMER_CLIENT_STATUS(1i32);
pub const UI_ANIMATION_TIMER_CLIENT_IDLE: UI_ANIMATION_TIMER_CLIENT_STATUS = UI_ANIMATION_TIMER_CLIENT_STATUS(0i32);
pub const UI_ANIMATION_UPDATE_NO_CHANGE: UI_ANIMATION_UPDATE_RESULT = UI_ANIMATION_UPDATE_RESULT(0i32);
pub const UI_ANIMATION_UPDATE_VARIABLES_CHANGED: UI_ANIMATION_UPDATE_RESULT = UI_ANIMATION_UPDATE_RESULT(1i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UI_ANIMATION_DEPENDENCIES(pub i32);
impl windows_core::TypeKind for UI_ANIMATION_DEPENDENCIES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UI_ANIMATION_DEPENDENCIES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_DEPENDENCIES").field(&self.0).finish()
    }
}
impl UI_ANIMATION_DEPENDENCIES {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for UI_ANIMATION_DEPENDENCIES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for UI_ANIMATION_DEPENDENCIES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for UI_ANIMATION_DEPENDENCIES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for UI_ANIMATION_DEPENDENCIES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for UI_ANIMATION_DEPENDENCIES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UI_ANIMATION_IDLE_BEHAVIOR(pub i32);
impl windows_core::TypeKind for UI_ANIMATION_IDLE_BEHAVIOR {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UI_ANIMATION_IDLE_BEHAVIOR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_IDLE_BEHAVIOR").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UI_ANIMATION_MANAGER_STATUS(pub i32);
impl windows_core::TypeKind for UI_ANIMATION_MANAGER_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UI_ANIMATION_MANAGER_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_MANAGER_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UI_ANIMATION_MODE(pub i32);
impl windows_core::TypeKind for UI_ANIMATION_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UI_ANIMATION_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UI_ANIMATION_PRIORITY_EFFECT(pub i32);
impl windows_core::TypeKind for UI_ANIMATION_PRIORITY_EFFECT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UI_ANIMATION_PRIORITY_EFFECT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_PRIORITY_EFFECT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UI_ANIMATION_REPEAT_MODE(pub i32);
impl windows_core::TypeKind for UI_ANIMATION_REPEAT_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UI_ANIMATION_REPEAT_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_REPEAT_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UI_ANIMATION_ROUNDING_MODE(pub i32);
impl windows_core::TypeKind for UI_ANIMATION_ROUNDING_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UI_ANIMATION_ROUNDING_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_ROUNDING_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UI_ANIMATION_SCHEDULING_RESULT(pub i32);
impl windows_core::TypeKind for UI_ANIMATION_SCHEDULING_RESULT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UI_ANIMATION_SCHEDULING_RESULT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_SCHEDULING_RESULT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UI_ANIMATION_SLOPE(pub i32);
impl windows_core::TypeKind for UI_ANIMATION_SLOPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UI_ANIMATION_SLOPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_SLOPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UI_ANIMATION_STORYBOARD_STATUS(pub i32);
impl windows_core::TypeKind for UI_ANIMATION_STORYBOARD_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UI_ANIMATION_STORYBOARD_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_STORYBOARD_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UI_ANIMATION_TIMER_CLIENT_STATUS(pub i32);
impl windows_core::TypeKind for UI_ANIMATION_TIMER_CLIENT_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UI_ANIMATION_TIMER_CLIENT_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_TIMER_CLIENT_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UI_ANIMATION_UPDATE_RESULT(pub i32);
impl windows_core::TypeKind for UI_ANIMATION_UPDATE_RESULT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UI_ANIMATION_UPDATE_RESULT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UI_ANIMATION_UPDATE_RESULT").field(&self.0).finish()
    }
}
pub const UIAnimationManager: windows_core::GUID = windows_core::GUID::from_u128(0x4c1fc63a_695c_47e8_a339_1a194be3d0b8);
pub const UIAnimationManager2: windows_core::GUID = windows_core::GUID::from_u128(0xd25d8842_8884_4a4a_b321_091314379bdd);
pub const UIAnimationTimer: windows_core::GUID = windows_core::GUID::from_u128(0xbfcd4a0c_06b6_4384_b768_0daa792c380e);
pub const UIAnimationTransitionFactory: windows_core::GUID = windows_core::GUID::from_u128(0x8a9b1cdd_fcd7_419c_8b44_42fd17db1887);
pub const UIAnimationTransitionFactory2: windows_core::GUID = windows_core::GUID::from_u128(0x84302f97_7f7b_4040_b190_72ac9d18e420);
pub const UIAnimationTransitionLibrary: windows_core::GUID = windows_core::GUID::from_u128(0x1d6322ad_aa85_4ef5_a828_86d71067d145);
pub const UIAnimationTransitionLibrary2: windows_core::GUID = windows_core::GUID::from_u128(0x812f944a_c5c8_4cd9_b0a6_b3da802f228d);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UI_ANIMATION_KEYFRAME(pub isize);
impl Default for UI_ANIMATION_KEYFRAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UI_ANIMATION_KEYFRAME {
    type TypeKind = windows_core::CopyType;
}
