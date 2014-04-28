#pragma once

#include "sequence.hpp"
#include <type_traits>

// In order to allow synthesis of operators for your class, specialise this template and provide
// a list of members to compare on.  Use SYNTH_MEMBER_ACCESSOR to create such a list.
template <typename T>
struct enable_operator_synthesis;

namespace detail {

// Create an accessor for the pointed-to-member.
template <typename MP, MP Ptr>
struct member_accessor {
    template <typename T>
    static auto get(T const& x) -> decltype(x.*Ptr) {
        return x.*Ptr;
    }
};

// Helper macro to avoid writing out the type.
#define SYNTH_MEMBER_ACCESSOR(x) ::detail::member_accessor<decltype(&x), &x>

template <typename Head, typename Tail>
struct meta_equality_op {
    using type = struct {
        template <typename T>
        static bool invoke(T const& lhs, T const& rhs) {
            if (Head::get(lhs) != Head::get(rhs))
                return false;
            return Tail::invoke(lhs, rhs);
        }
    };
};

struct meta_equality_base {
    template <typename T>
    static bool invoke(T const&, T const&) {
        return true;
    }
};

template <typename Seq>
using synthesise_equality = foldr<meta_equality_op, meta_equality_base, Seq>;

template <typename Head, typename Tail>
struct meta_lessthan_op {
    using type = struct {
        template <typename T>
        static bool invoke(T const& lhs, T const& rhs) {
            auto const& lhs_head = Head::get(lhs);
            auto const& rhs_head = Head::get(rhs);
            if (lhs_head < rhs_head)
                return true;
            if (lhs_head != rhs_head)
                return false;
            return Tail::invoke(lhs, rhs);
        }
    };
};

struct meta_lessthan_base {
    template <typename T>
    static bool invoke(T const&, T const&) {
        return false;
    }
};

template <typename Seq>
using synthesise_lessthan = foldr<meta_lessthan_op, meta_lessthan_base, Seq>;

template <typename T>
using bool_if_enabled = typename std::enable_if<enable_operator_synthesis<T>::value, bool>::type;
}

// By default, do not allow synthesis of operators.
template <typename T>
struct enable_operator_synthesis : std::false_type {};

template <typename T>
detail::bool_if_enabled<T> operator==(T const& lhs, T const& rhs) {
    using members = typename enable_operator_synthesis<T>::members;
    return detail::synthesise_equality<members>::invoke(lhs, rhs);
}

template <typename T>
detail::bool_if_enabled<T> operator!=(T const& lhs, T const& rhs) {
    return !(lhs == rhs);
}

template <typename T>
detail::bool_if_enabled<T> operator<(T const& lhs, T const& rhs) {
    using members = typename enable_operator_synthesis<T>::members;
    return detail::synthesise_lessthan<members>::invoke(lhs, rhs);
}

template <typename T>
detail::bool_if_enabled<T> operator<=(T const& lhs, T const& rhs) {
    return !(rhs > lhs);
}

template <typename T>
detail::bool_if_enabled<T> operator>(T const& lhs, T const& rhs) {
    return rhs < lhs;
}

template <typename T>
detail::bool_if_enabled<T> operator>=(T const& lhs, T const& rhs) {
    return rhs <= lhs;
}

