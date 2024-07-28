use crate::extractors::state::Params;

pub fn handle_gas_timing(params: &mut Params) {
    if params.gas == params.max_gas {
        params.refilling_in = 0;
    } else if params.refilling_in == 0 {
        params.refilling_in = 90;
    } else {
        params.refilling_in -= 1;
        if params.refilling_in == 0 {
            params.gas += 1
        };
    }
}
