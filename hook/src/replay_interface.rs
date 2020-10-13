use crate::POINTERS;

#[repr(C)]
pub struct CReplayInterface {
    pub pad_0000: [u8; 16],
    pub vehicle_interface: *mut CVehicleInterface,
    pub ped_interface: *mut CPedInterface,
    pub pad_0020: [u8; 8],
    pub object_interface: *mut CObjectInterface,
    pub pad_0030: [u8; 16],
}

#[repr(C)]
pub struct CObjectInterface {
    pub pad_0000: [u8; 344],
    pub object_list: *mut CObjectList,
    pub max_objects: i32,
    pub pad_0164: [u8; 4],
    pub current_objects: i32,
}

impl CObjectInterface {
    fn _get_object(&self, index: usize) -> *mut CObject {
        if index < self.max_objects as usize {
            unsafe {
                return (*self.object_list).object_handles[index].object;
            }
        }

        std::ptr::null_mut()
    }
}

#[repr(C)]
pub struct CObjectList {
    pub object_handles: [CObjectHandle; 2300],
}

#[repr(C)]
pub struct CObjectHandle {
    pub object: *mut CObject,
    pub handle: i32,
    pub pad_000c: [u8; 4],
}

#[repr(C)]
pub struct CObject {
    pub pad_0000: [u8; 72],
}

#[repr(C)]
pub struct CPedInterface {
    pub pad_0000: [u8; 256],
    pub ped_list: *mut CPedList,
    pub max_peds: i32,
    pub pad_010c: [u8; 4],
    pub current_peds: i32,
}

impl CPedInterface {
    fn get_ped(&self, index: usize) -> *mut CPed {
        if index < self.max_peds as usize {
            unsafe {
                return (*self.ped_list).ped_handles[index].ped;
            }
        }

        std::ptr::null_mut()
    }
}

#[repr(C)]
pub struct CPedList {
    pub ped_handles: [CPedHandle; 256],
    pub pad_1000: [u8; 64],
}

#[repr(C)]
pub struct CPedHandle {
    pub ped: *mut CPed,
    pub handle: i32,
    pub pad_000c: [u8; 4],
}

#[repr(C)]
pub struct CPed {}

#[repr(C)]
pub struct CVehicleInterface {
    pub pad_0000: [u8; 384],
    pub vehicle_list: *mut CVehicleList,
    pub max_vehicles: i32,
    pub pad_018c: [u8; 4],
    pub current_vehicles: i32,
}

impl CVehicleInterface {
    fn get_vehicle(&self, index: usize) -> *mut CVehicle {
        if index < self.max_vehicles as usize {
            unsafe {
                return (*self.vehicle_list).vehicle_handles[index].vehicle;
            }
        }

        std::ptr::null_mut()
    }
}

#[repr(C)]
pub struct CVehicleList {
    pub vehicle_handles: [CVehicleHandle; 300],
}

#[repr(C)]
pub struct CVehicleHandle {
    pub vehicle: *mut CVehicle,
    pub handle: i32,
    pub pad_000c: [u8; 4],
}

#[repr(C)]
pub struct CVehicle {}

pub fn get_all_vehicles() -> (i32, Vec<i32>) {
    let mut vehicles: Vec<i32> = vec![];

    unsafe {
        let max_vehicles = (*(*POINTERS.replay_interface).vehicle_interface).max_vehicles;

        for i in 0..max_vehicles {
            let vehicle = (*(*POINTERS.replay_interface).vehicle_interface).get_vehicle(i as usize);

            let handle = (*(*(*POINTERS.replay_interface).vehicle_interface).vehicle_list)
                .vehicle_handles[i as usize]
                .handle;

            if handle == 65535 {
                continue;
            }

            vehicles.push((POINTERS.address_to_entity.unwrap())(
                vehicle as *mut std::ffi::c_void,
            ));
        }

        (
            (*(*POINTERS.replay_interface).vehicle_interface).current_vehicles,
            vehicles,
        )
    }
}

pub fn get_all_peds() -> (i32, Vec<i32>) {
    let mut peds: Vec<i32> = vec![];

    unsafe {
        let max_peds = (*(*POINTERS.replay_interface).ped_interface).max_peds;

        for i in 0..max_peds {
            let ped = (*(*POINTERS.replay_interface).ped_interface).get_ped(i as usize);

            let handle = (*(*(*POINTERS.replay_interface).ped_interface).ped_list).ped_handles
                [i as usize]
                .handle;

            if handle == 65535 {
                continue;
            }

            peds.push((POINTERS.address_to_entity.unwrap())(
                ped as *mut std::ffi::c_void,
            ));
        }

        (
            (*(*POINTERS.replay_interface).ped_interface).current_peds,
            peds,
        )
    }
}
