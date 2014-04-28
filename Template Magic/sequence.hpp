#pragma once

template <typename... Elems>
struct sequence;

template <typename Head, typename... Tail>
struct sequence<Head, Tail...> {
    using head = Head;
    using tail = sequence<Tail...>;
};

namespace detail {
template <typename T, typename Seq>
struct cons_impl;

template <template <typename, typename> class Op, typename Initial, typename Seq>
struct foldr_impl;
}

template <typename Head, typename Seq>
using cons = typename detail::cons_impl<Head, Seq>::type;

template <template <typename, typename> class Op, typename Initial, typename Seq>
using foldr = typename detail::foldr_impl<Op, Initial, Seq>::type;

namespace detail {
template <typename Head, typename... Tail>
struct cons_impl<Head, sequence<Tail...>> {
    using type = sequence<Head, Tail...>;
};

template <template <typename, typename> class Op, typename Initial, typename Seq>
struct foldr_impl : Op<typename Seq::head, foldr<Op, Initial, typename Seq::tail>> {};

template <template <typename, typename> class Op, typename Initial>
struct foldr_impl<Op, Initial, sequence<>> {
    using type = Initial;
};
}
