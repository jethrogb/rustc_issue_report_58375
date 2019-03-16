mod std_facade {
    pub use std::borrow::ToOwned;
    pub use std::boxed::Box;
    pub use std::fmt;
    pub use std::rc::Rc;
    pub use std::string::String;
    pub use std::sync::Arc;
    pub use std::vec::Vec;
}
pub mod arbitrary {
    use crate::strategy::statics;
    mod traits {
        use crate::strategy::Strategy;
        use core::fmt;
        pub trait Arbitrary: Sized + fmt::Debug {
            type Parameters: Default;
            fn arbitrary() -> Self::Strategy {
                Self::arbitrary_with(Default::default())
            }
            fn arbitrary_with(args: Self::Parameters) -> Self::Strategy;
            type Strategy: Strategy;
        }
        pub type StrategyFor<A> = <A as Arbitrary>::Strategy;
        pub type ParamsFor<A> = <A as Arbitrary>::Parameters;
        pub fn any<A: Arbitrary>() -> StrategyFor<A> {
            A::arbitrary()
        }
        pub fn any_with<A: Arbitrary>(args: ParamsFor<A>) -> StrategyFor<A> {
            A::arbitrary_with(args)
        }
        use crate::num::{u128, u16, u32, u64, u8, usize};
        impl crate::arbitrary::Arbitrary for u16 {
            type Parameters = ();
            type Strategy = u16::Any;
            fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
                u16::ANY
            }
        }
        mod char {
            use core::char::*;
            const VEC_MAX: usize = core::u16::MAX as usize;
            use crate::arbitrary::*;
            use crate::strategy::statics::static_map;
            impl crate::arbitrary::Arbitrary for DecodeUtf16<<Vec<u16> as IntoIterator>::IntoIter> {
                type Parameters = ();
                type Strategy = SMapped<Vec<u16>, Self>;
                fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
                    static_map(vec(any::<u16>(), ..VEC_MAX), decode_utf16)
                }
            }
            use crate::collection::*;
            type RangedParams1<A> = (SizeRange, A);
            impl<A: Arbitrary> crate::arbitrary::Arbitrary for Vec<A> {
                type Parameters = RangedParams1<A::Parameters>;
                type Strategy = VecStrategy<A::Strategy>;
                fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
                    let (range, a) = args;
                    vec(any_with::<A>(a), range)
                }
            }
        }
    }
    pub use self::traits::*;
    pub type SMapped<I, O> = statics::Map<StrategyFor<I>, fn(I) -> O>;
}
pub mod bits {
    use crate::std_facade::{fmt, Vec};
    use bit_set::BitSet;
    pub trait BitSetLike: Clone + fmt::Debug {
        fn new_bitset(max: usize) -> Self;
        fn len(&self) -> usize;
        fn test(&self, ix: usize) -> bool;
        fn set(&mut self, ix: usize);
        fn clear(&mut self, ix: usize);
        fn count(&self) -> usize {
            let mut n = 0;
            n
        }
    }
    impl BitSetLike for BitSet {
        fn new_bitset(max: usize) -> Self {
            BitSet::with_capacity(max)
        }
        fn len(&self) -> usize {
            self.capacity()
        }
        fn test(&self, bit: usize) -> bool {
            self.contains(bit)
        }
        fn set(&mut self, bit: usize) {}
        fn clear(&mut self, bit: usize) {}
    }
    pub mod varsize {
        use super::*;
        use core::iter::FromIterator;
        type Inner = BitSet;
        pub struct VarBitSet(Inner);
        impl core::fmt::Debug for VarBitSet {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match *self {
                    VarBitSet(ref __self_0_0) => {
                        let mut debug_trait_builder = f.debug_tuple("VarBitSet");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        impl core::clone::Clone for VarBitSet {
            fn clone(&self) -> VarBitSet {
                match *self {
                    VarBitSet(ref __self_0_0) => VarBitSet(core::clone::Clone::clone(&*__self_0_0)),
                }
            }
        }
        impl VarBitSet {
            pub fn saturated(len: usize) -> Self {
                (0..len).collect::<VarBitSet>()
            }
        }
        impl BitSetLike for VarBitSet {
            fn new_bitset(max: usize) -> Self {
                VarBitSet(Inner::new_bitset(max))
            }
            fn len(&self) -> usize {
                BitSetLike::len(&self.0)
            }
            fn test(&self, bit: usize) -> bool {
                BitSetLike::test(&self.0, bit)
            }
            fn set(&mut self, bit: usize) {}
            fn clear(&mut self, bit: usize) {}
        }
        impl FromIterator<usize> for VarBitSet {
            fn from_iter<T: IntoIterator>(iter: T) -> Self {
                let mut bits = VarBitSet::new_bitset(0);
                bits
            }
        }
    }
    pub use self::varsize::VarBitSet;
}
pub mod collection {
    use crate::bits::{BitSetLike, VarBitSet};
    use crate::num::sample_uniform_incl;
    use crate::strategy::*;
    use crate::test_runner::*;
    use core::ops::{Add, Range, RangeInclusive, RangeTo, RangeToInclusive};
    pub struct SizeRange(Range<usize>);
    pub fn size_range(from: impl Into<SizeRange>) -> SizeRange {
        from.into()
    }
    impl Default for SizeRange {
        fn default() -> Self {
            size_range(0..100)
        }
    }
    impl SizeRange {
        pub fn start(&self) -> usize {
            self.0.start
        }
        pub fn start_end_incl(&self) -> (usize, usize) {
            (self.start(), self.end_incl())
        }
        pub fn end_incl(&self) -> usize {
            self.0.end - 1
        }
    }
    impl From<RangeTo<usize>> for SizeRange {
        fn from(high: RangeTo<usize>) -> Self {
            size_range(0..high.end)
        }
    }
    impl From<Range<usize>> for SizeRange {
        fn from(r: Range<usize>) -> Self {
            SizeRange(r)
        }
    }
    pub struct VecStrategy<T: Strategy> {
        element: T,
        size: SizeRange,
    }
    impl<T: core::fmt::Debug + Strategy> core::fmt::Debug for VecStrategy<T> {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match *self {
                VecStrategy {
                    element: ref __self_0_0,
                    size: ref __self_0_1,
                } => {
                    let mut debug_trait_builder = f.debug_struct("VecStrategy");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    pub fn vec<T: Strategy>(element: T, size: impl Into<SizeRange>) -> VecStrategy<T> {
        let size = size.into();
        VecStrategy { element, size }
    }
    enum Shrink {
        DeleteElement(usize),
    }
    pub struct VecValueTree<T: ValueTree> {
        elements: Vec<T>,
        included_elements: VarBitSet,
        min_size: usize,
        shrink: Shrink,
        prev_shrink: Option<Shrink>,
    }
    impl<T: Strategy> Strategy for VecStrategy<T> {
        type Tree = VecValueTree<T::Tree>;
        type Value = Vec<T::Value>;
        fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
            let (start, end) = self.size.start_end_incl();
            let max_size = sample_uniform_incl(runner, start, end);
            let mut elements = Vec::with_capacity(max_size);
            if elements.len() < max_size {}
            Ok(VecValueTree {
                elements,
                included_elements: VarBitSet::saturated(max_size),
                min_size: start,
                shrink: Shrink::DeleteElement(0),
                prev_shrink: None,
            })
        }
    }
    impl<T: ValueTree> ValueTree for VecValueTree<T> {
        type Value = Vec<T::Value>;
        fn current(&self) -> Vec<T::Value> {
            self.elements
                .iter()
                .enumerate()
                .filter(|&(ix, _)| self.included_elements.test(ix))
                .map(|(_, element)| element.current())
                .collect()
        }
        fn simplify(&mut self) -> bool {
            if let Shrink::DeleteElement(ix) = self.shrink {
                if ix == self.min_size {}
            }
            unimplemented!()
        }
        fn complicate(&mut self) -> bool {
            match self.prev_shrink {
                None => false,
                Some(Shrink::DeleteElement(ix)) => true,
            }
        }
    }
}
pub mod num {
    use crate::test_runner::TestRunner;
    use rand::distributions::uniform::{SampleUniform, Uniform};
    use rand::distributions::{Distribution, Standard};
    pub fn sample_uniform_incl<X: SampleUniform>(run: &mut TestRunner, start: X, end: X) -> X {
        Uniform::new_inclusive(start, end).sample(run.rng())
    }
    pub mod u8 {}
    pub mod u16 {
        use crate::strategy::*;
        use crate::test_runner::TestRunner;
        use rand::Rng;
        pub struct Any(());
        impl core::fmt::Debug for Any {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match *self {
                    Any(ref __self_0_0) => {
                        let mut debug_trait_builder = f.debug_tuple("Any");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        pub const ANY: Any = Any(());
        impl Strategy for Any {
            type Tree = BinarySearch;
            type Value = u16;
            fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
                Ok(BinarySearch::new(runner.rng().gen()))
            }
        }
        pub struct BinarySearch {
            lo: u16,
            curr: u16,
            hi: u16,
        }
        impl BinarySearch {
            pub fn new(start: u16) -> Self {
                BinarySearch {
                    lo: 0,
                    curr: start,
                    hi: start,
                }
            }
            fn reposition(&mut self) -> bool {
                let interval = self.hi - self.lo;
                let new_mid = self.lo + interval / 2;
                if new_mid == self.curr {
                    false
                } else {
                    true
                }
            }
        }
        impl ValueTree for BinarySearch {
            type Value = u16;
            fn current(&self) -> u16 {
                self.curr
            }
            fn simplify(&mut self) -> bool {
                self.reposition()
            }
            fn complicate(&mut self) -> bool {
                self.reposition()
            }
        }
    }
    pub mod u32 {}
    pub mod u64 {}
    pub mod u128 {}
    pub mod usize {
        pub struct Any;
        impl core::clone::Clone for Any {
            fn clone(&self) -> Any {
                *self
            }
        }
        impl core::marker::Copy for Any {}
    }
}
pub mod strategy {
    mod traits {
        use crate::std_facade::{fmt, Arc, Box, Rc};
        use crate::strategy::*;
        use crate::test_runner::*;
        pub type NewTree<S> = Result<<S as Strategy>::Tree, Reason>;
        pub trait Strategy: fmt::Debug {
            type Tree: ValueTree<Value = Self::Value>;
            type Value: fmt::Debug;
            fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self>;
            fn prop_perturb<O: fmt::Debug, F>(self, fun: F) -> Perturb<Self, F>
            where
                Self: Sized,
            {
                Perturb {
                    source: self,
                    fun: Arc::new(fun),
                }
            }
            fn prop_recursive<R: Strategy + 'static, F: Fn(BoxedStrategy<Self::Value>) -> R>(
                self,
                depth: u32,
                desired_size: u32,
                expected_branch_size: u32,
                recurse: F,
            ) -> Recursive<Self::Value, F>
            where
                Self: Sized + 'static,
            {
                Recursive::new(self, depth, desired_size, expected_branch_size, recurse)
            }
            fn boxed(self) -> BoxedStrategy<Self::Value>
            where
                Self: Sized + 'static,
            {
                BoxedStrategy(Arc::new(BoxedStrategyWrapper(self)))
            }
            fn sboxed(self) -> SBoxedStrategy<Self::Value>
            where
                Self: Sized + Send + Sync + 'static,
            {
                SBoxedStrategy(Arc::new(BoxedStrategyWrapper(self)))
            }
        }
        pub trait ValueTree {
            type Value: fmt::Debug;
            fn current(&self) -> Self::Value;
            fn simplify(&mut self) -> bool;
            fn complicate(&mut self) -> bool;
        }
        impl<T: ValueTree + ?Sized> ValueTree for Box<T> {
            type Value = T::Value;
            fn current(&self) -> Self::Value {
                (**self).current()
            }
            fn simplify(&mut self) -> bool {
                (**self).simplify()
            }
            fn complicate(&mut self) -> bool {
                (**self).complicate()
            }
        }
        type BoxedVT<T> = Box<dyn ValueTree<Value = T>>;
        pub struct BoxedStrategy<T>(Arc<dyn Strategy<Value = T, Tree = BoxedVT<T>>>);
        pub struct SBoxedStrategy<T>(Arc<dyn Strategy<Value = T, Tree = BoxedVT<T>> + Sync + Send>);
        struct BoxedStrategyWrapper<T>(T);
        impl<T: core::fmt::Debug> core::fmt::Debug for BoxedStrategyWrapper<T> {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match *self {
                    BoxedStrategyWrapper(ref __self_0_0) => {
                        let mut debug_trait_builder = f.debug_tuple("BoxedStrategyWrapper");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        impl<T: Strategy> Strategy for BoxedStrategyWrapper<T>
        where
            T::Tree: 'static,
        {
            type Tree = Box<dyn ValueTree<Value = T::Value>>;
            type Value = T::Value;
            fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
                Ok(Box::new(self.0.new_tree(runner)?))
            }
        }
    }
    mod map {
        use crate::std_facade::Arc;
        use core::marker::PhantomData;
        pub struct Perturb<S, F> {
            pub source: S,
            pub fun: Arc<F>,
        }
    }
    mod recursive {
        use crate::std_facade::{fmt, Arc, Box, Vec};
        use crate::strategy::traits::*;
        pub struct Recursive<T, F> {
            pd: std::marker::PhantomData<(T, F)>,
        }
        impl<T: fmt::Debug + 'static, R: Strategy + 'static, F: Fn(BoxedStrategy<T>) -> R> Recursive<T, F> {
            pub fn new(
                base: impl Strategy<Value = T> + 'static,
                depth: u32,
                desired_size: u32,
                expected_branch_size: u32,
                recurse: F,
            ) -> Self {
                Self {
                    pd: std::marker::PhantomData,
                }
            }
        }
    }
    pub use self::map::*;
    pub use self::recursive::*;
    pub use self::traits::*;
    pub mod statics {
        use crate::std_facade::fmt;
        use crate::strategy::traits::*;
        use crate::test_runner::*;
        pub trait MapFn<T> {
            type Output: fmt::Debug;
            fn apply(&self, t: T) -> Self::Output;
        }
        pub struct Map<S, F> {
            source: S,
            fun: F,
        }
        impl<S, F> Map<S, F> {
            pub fn new(source: S, fun: F) -> Self {
                Map { source, fun }
            }
        }
        impl<S: fmt::Debug, F> fmt::Debug for Map<S, F> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_struct("Map")
                    .field("sourcefun", &"function")
                    .finish()
            }
        }
        impl<S: Strategy, F: Clone + MapFn<S::Value>> Strategy for Map<S, F> {
            type Tree = Map<S::Tree, F>;
            type Value = F::Output;
            fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
                self.source.new_tree(runner).map(|v| Map {
                    source: v,
                    fun: self.fun.clone(),
                })
            }
        }
        impl<S: ValueTree, F: MapFn<S::Value>> ValueTree for Map<S, F> {
            type Value = F::Output;
            fn current(&self) -> F::Output {
                self.fun.apply(self.source.current())
            }
            fn simplify(&mut self) -> bool {
                self.source.simplify()
            }
            fn complicate(&mut self) -> bool {
                self.source.complicate()
            }
        }
        impl<I, O: fmt::Debug> MapFn<I> for fn(I) -> O {
            type Output = O;
            fn apply(&self, x: I) -> Self::Output {
                self(x)
            }
        }
        pub fn static_map<S: Strategy, O: fmt::Debug>(
            strat: S,
            fun: fn(S::Value) -> O,
        ) -> Map<S, fn(S::Value) -> O> {
            Map::new(strat, fun)
        }
    }
}
pub mod test_runner {
    mod rng {
        use rand::{self, Rng, RngCore, SeedableRng};
        pub enum RngAlgorithm {
            XorShift,
            ChaCha,
            PassThrough,
            _NonExhaustive,
        }
        impl core::clone::Clone for RngAlgorithm {
            fn clone(&self) -> RngAlgorithm {
                *self
            }
        }
        impl core::marker::Copy for RngAlgorithm {}
        pub struct TestRng;
        impl RngCore for TestRng {
            fn next_u32(&mut self) -> u32 {
                unimplemented!()
            }
            fn next_u64(&mut self) -> u64 {
                unimplemented!()
            }
            fn fill_bytes(&mut self, dest: &mut [u8]) {
                unimplemented!()
            }
            fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
                unimplemented!()
            }
        }
    }
    mod reason {
        pub struct Reason;
    }
    mod runner {
        use crate::test_runner::TestRng;
        pub struct TestRunner {
            rng: TestRng,
        }
        impl TestRunner {
            pub fn rng(&mut self) -> &mut TestRng {
                &mut self.rng
            }
        }
    }
    pub use self::reason::*;
    pub use self::rng::*;
    pub use self::runner::*;
}
