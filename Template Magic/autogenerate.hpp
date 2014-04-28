#pragma once

#include <boost/mpl/placeholders.hpp>
#include <boost/mpl/reverse_fold.hpp>
#include <type_traits>

// In order to allow synthesis of operators for your class, specialise this template and provide
// a list of members to compare on.  Use SYNTH_MEMBER_ACCESSOR to create such a list.
template <typename T>
struct enable_operator_synthesis;

namespace detail {
using namespace boost::mpl::placeholders;
namespace mpl = boost::mpl;

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

template <bool Value>
struct compare_as_value {
    template <typename T>
    static bool invoke(T const&, T const&) {
        return Value;
    }
};

using compare_true = compare_as_value<true>;
using compare_false = compare_as_value<false>;

template <typename Getter, typename Rest>
struct compare_equal {
    template <typename T>
    static bool invoke(T const& lhs, T const& rhs) {
        if (Getter::get(lhs) != Getter::get(rhs))
            return false;
        return Rest::invoke(lhs, rhs);
    }
};

template <typename Getter, typename Rest>
struct compare_lessthan {
    template <typename T>
    static bool invoke(T const& lhs, T const& rhs) {
        auto const& lhs_val = Getter::get(lhs);
        auto const& rhs_val = Getter::get(rhs);
        if (lhs_val < rhs_val)
            return true;
        if (lhs_val != rhs_val)
            return false;
        return Rest::invoke(lhs, rhs);
    }
};

template <typename Seq>
using synthesise_equality = typename mpl::reverse_fold<Seq, compare_true, compare_equal<_2, _1>>::type;

template <typename Seq>
using synthesise_lessthan = typename mpl::reverse_fold<Seq, compare_false, compare_lessthan<_2, _1>>::type;

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

