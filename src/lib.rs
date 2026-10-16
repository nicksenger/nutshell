#![no_std]

use core::marker::PhantomData;

pub struct Empty;
pub struct List<T>(PhantomData<T>);

pub struct UTerm;
pub struct B0;
pub struct B1;
pub struct UInt<U, B>(PhantomData<(U, B)>);
type U0 = UInt<UTerm, B0>;

pub trait Inc {
    type Out;
}
impl Inc for UTerm {
    type Out = UInt<UTerm, B1>;
}
impl<U> Inc for UInt<U, B0> {
    type Out = UInt<U, B1>;
}
impl<U> Inc for UInt<U, B1>
where
    U: Inc,
{
    type Out = UInt<<U as Inc>::Out, B0>;
}

pub struct C0;
pub struct CN<T>(PhantomData<T>);

type U8 = CN<CN<CN<CN<CN<CN<CN<CN<C0>>>>>>>>;
type U48 = CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<CN<C0>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>;

pub trait Mappend {
    type Out;
}
impl Mappend for (Empty, Empty) {
    type Out = Empty;
}
impl<A, B> Mappend for (Empty, List<(A, B)>) {
    type Out = List<(A, B)>;
}
impl<A, B> Mappend for (List<(A, B)>, Empty) {
    type Out = List<(A, B)>;
}
impl<A, B, C, D> Mappend for (List<(A, B)>, List<(C, D)>)
where
    (B, List<(C, D)>): Mappend,
{
    type Out = List<(A, <(B, List<(C, D)>) as Mappend>::Out)>;
}

pub trait Rev {
    type Out;
}
impl Rev for Empty {
    type Out = Empty;
}
impl<H, T> Rev for List<(H, T)>
where
    T: Rev,
    (<T as Rev>::Out, List<(H, Empty)>): Mappend,
{
    type Out = <(<T as Rev>::Out, List<(H, Empty)>) as Mappend>::Out;
}

type Nxt<C> = <C as Inc>::Out;

pub trait BuildElem {
    type Out;
    type Next;
}
impl<E> BuildElem for (E, C0) {
    type Out = Empty;
    type Next = E;
}
impl<C, T> BuildElem for (C, CN<T>)
where
    C: Inc,
    (Nxt<C>, T): BuildElem,
{
    type Out = List<(C, <(Nxt<C>, T) as BuildElem>::Out)>;
    type Next = <(Nxt<C>, T) as BuildElem>::Next;
}

pub trait BuildMid<P> {
    type Out;
    type Next;
}
impl<E, P> BuildMid<P> for (E, C0) {
    type Out = Empty;
    type Next = E;
}
type Col<C, P> = <(C, P) as BuildElem>::Out;
type ColNxt<C, P> = <(C, P) as BuildElem>::Next;
impl<C, T, P> BuildMid<P> for (C, CN<T>)
where
    (C, P): BuildElem,
    (ColNxt<C, P>, T): BuildMid<P>,
{
    type Out = List<(Col<C, P>, <(ColNxt<C, P>, T) as BuildMid<P>>::Out)>;
    type Next = <(ColNxt<C, P>, T) as BuildMid<P>>::Next;
}

pub trait Powder<N, P> {
    type Out;
    type Next;
}
impl<E, N, P> Powder<N, P> for (E, C0) {
    type Out = Empty;
    type Next = E;
}
type Row<C, N, P> = <(C, N) as BuildMid<P>>::Out;
type RowNxt<C, N, P> = <(C, N) as BuildMid<P>>::Next;
impl<C, T, N, P> Powder<N, P> for (C, CN<T>)
where
    (C, N): BuildMid<P>,
    (RowNxt<C, N, P>, T): Powder<N, P>,
{
    type Out = List<(Row<C, N, P>, <(RowNxt<C, N, P>, T) as Powder<N, P>>::Out)>;
    type Next = <(RowNxt<C, N, P>, T) as Powder<N, P>>::Next;
}

pub trait Chain {
    type Out;
}
impl Chain for Empty {
    type Out = Empty;
}
impl<H, T> Chain for List<(H, T)>
where
    H: Rev,
    T: Chain,
{
    type Out = List<(<H as Rev>::Out, <T as Chain>::Out)>;
}

pub trait Shell {
    type Tap;
}
impl Shell for Empty {
    type Tap = Empty;
}
impl<H, T> Shell for List<(H, T)>
where
    H: Chain,
    T: Shell,
{
    type Tap = List<(<H as Chain>::Out, <T as Shell>::Tap)>;
}

type Fuse = <(U8, U8) as Powder<U8, U48>>::Out;
pub fn det(_: <Fuse as Shell>::Tap) {}
