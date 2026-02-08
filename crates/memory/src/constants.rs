pub(crate) enum ClientAddress {
    CameraZoom = 0x1858620,
    CameraCrywolf = 0x1588FC,
}

pub(crate) enum EntityAddress {
    Nickname = 0x1589F1,
    MapID = 0x18585EC,
}

pub(crate) const MAPS_DISABLED: [i16; 7] = [73, 74, 75, 76, 78, 93, 94];