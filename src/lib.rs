mixed_struct_proc::mixed_struct_proc!();

//create items with the approriate Mixed structs so the generics can be inferred
#[macro_export]
macro_rules! mixed_array {
    () => {
        []
    };
    ($a:expr $(,)?) => {
        [$crate::Mixed1::A(&a)]
    };
    ($a:expr, $b:expr $(,)?) => {
        [$crate::Mixed2::A($a), $crate::Mixed2::B($b)]
    };
    ($a:expr, $b:expr, $c:expr $(,)?) => {
        [
            $crate::Mixed3::A($a),
            $crate::Mixed3::B($b),
            $crate::Mixed3::C($c),
        ]
    };
    ($a:expr, $b:expr, $c:expr, $d:expr $(,)?) => {
        [
            $crate::Mixed4::A($a),
            $crate::Mixed4::B($b),
            $crate::Mixed4::C($c),
            $crate::Mixed4::D($d),
        ]
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr $(,)?) => {
        [
            $crate::Mixed5::A($a),
            $crate::Mixed5::B($b),
            $crate::Mixed5::C($c),
            $crate::Mixed5::D($d),
            $crate::Mixed5::E($e),
        ]
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr $(,)?) => {
        [
            $crate::Mixed6::A($a),
            $crate::Mixed6::B($b),
            $crate::Mixed6::C($c),
            $crate::Mixed6::D($d),
            $crate::Mixed6::E($e),
            $crate::Mixed6::F($f),
        ]
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr $(,)?) => {
        [
            $crate::Mixed7::A($a),
            $crate::Mixed7::B($b),
            $crate::Mixed7::C($c),
            $crate::Mixed7::D($d),
            $crate::Mixed7::E($e),
            $crate::Mixed7::F($f),
            $crate::Mixed7::G($g),
        ]
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr $(,)?) => {
        [
            $crate::Mixed8::A($a),
            $crate::Mixed8::B($b),
            $crate::Mixed8::C($c),
            $crate::Mixed8::D($d),
            $crate::Mixed8::E($e),
            $crate::Mixed8::F($f),
            $crate::Mixed8::G($g),
            $crate::Mixed8::H($h),
        ]
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr $(,)?) => {
        [
            $crate::Mixed9::A($a),
            $crate::Mixed9::B($b),
            $crate::Mixed9::C($c),
            $crate::Mixed9::D($d),
            $crate::Mixed9::E($e),
            $crate::Mixed9::F($f),
            $crate::Mixed9::G($g),
            $crate::Mixed9::H($h),
            $crate::Mixed9::I($i),
        ]
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr, $j:expr $(,)?) => {
        [
            $crate::Mixed10::A($a),
            $crate::Mixed10::B($b),
            $crate::Mixed10::C($c),
            $crate::Mixed10::D($d),
            $crate::Mixed10::E($e),
            $crate::Mixed10::F($f),
            $crate::Mixed10::G($g),
            $crate::Mixed10::H($h),
            $crate::Mixed10::I($i),
            $crate::Mixed10::J($j),
        ]
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr, $j:expr, $k:expr $(,)?) => {
        [
            $crate::Mixed11::A($a),
            $crate::Mixed11::B($b),
            $crate::Mixed11::C($c),
            $crate::Mixed11::D($d),
            $crate::Mixed11::E($e),
            $crate::Mixed11::F($f),
            $crate::Mixed11::G($g),
            $crate::Mixed11::H($h),
            $crate::Mixed11::I($i),
            $crate::Mixed11::J($j),
            $crate::Mixed11::K($k),
        ]
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr, $j:expr, $k:expr, $l:expr $(,)?) => {
        [
            $crate::Mixed12::A($a),
            $crate::Mixed12::B($b),
            $crate::Mixed12::C($c),
            $crate::Mixed12::D($d),
            $crate::Mixed12::E($e),
            $crate::Mixed12::F($f),
            $crate::Mixed12::G($g),
            $crate::Mixed12::H($h),
            $crate::Mixed12::I($i),
            $crate::Mixed12::J($j),
            $crate::Mixed12::K($k),
            $crate::Mixed12::L($l),
        ]
    };
}
