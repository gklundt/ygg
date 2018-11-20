use crate::metrics::uom::DistanceKind;
use crate::metrics::uom::VelocityKind;
use crate::metrics::uom::TimeKind;

pub enum ResourceModifierKind {
    SpeedLimit { modifies: DistanceKind, unit: VelocityKind, produces: TimeKind }
}






//impl ResourceModifierKind {
//    pub fn apply_speed_limit(&mut self) -> &mut Self {
//        match self {
//            ResourceModifierKind::SpeedLimit {} =>
//                self.,
//            _ => self
//        }
//    }
//}

#[cfg(test)]
mod tests {
    use crate::metrics::uom::DistanceKind;
    use crate::metrics::uom::VelocityKind;
    use crate::heuristics::resource_modifiers::ResourceModifierKind;
    use crate::metrics::uom::TimeKind;

    #[test]
    fn test_something() {
        let s: DistanceKind = DistanceKind::Meters(1.0);
        let v: VelocityKind = VelocityKind::MetersPerSecond(10.0);
        let t: TimeKind = TimeKind::Seconds(0.0);

        let rm = ResourceModifierKind::SpeedLimit {modifies: s, unit: v, produces: t};

//        rm = rm.apply_speed_limit()
    }
}