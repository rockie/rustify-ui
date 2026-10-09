//! Pure Rust class fragments for typed component variants.

use std::borrow::Cow;

/// A typed class value whose unmerged fragments remain available to a scope.
pub trait IntoTailwindClass {
    /// Returns base, variant and size classes without resolving conflicts.
    fn raw_class(&self) -> Cow<'_, str>;

    /// Resolves these classes through the default cn engine.
    fn to_class(&self) -> String {
        super::merge(&[&self.raw_class()])
    }

    /// Adds caller classes last and resolves the combined input exactly once.
    fn with_class(&self, class: impl AsRef<str>) -> String {
        super::merge(&[&self.raw_class(), class.as_ref()])
    }
}

/// Generates class structs and enums from literal base/variant/size fragments.
#[macro_export]
macro_rules! class_variants {
    (@enum $name:ident { $first:ident: $first_class:literal $(, $key:ident: $class:literal)* $(,)? }) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum $name { $first, $($key,)* }

        impl Default for $name {
            fn default() -> Self { Self::$first }
        }

        impl $name {
            pub fn as_class(&self) -> &'static str {
                match self { Self::$first => $first_class, $(Self::$key => $class,)* }
            }
        }

        impl $crate::cn::class_variants::IntoTailwindClass for $name {
            fn raw_class(&self) -> ::std::borrow::Cow<'_, str> {
                ::std::borrow::Cow::Borrowed(self.as_class())
            }
        }
    };
    (
        $name:ident {
            base: $base:literal,
            variant: $variant:ident { $($variants:tt)* },
            size: $size:ident { $($sizes:tt)* }
        }
    ) => {
        $crate::class_variants!(@enum $variant { $($variants)* });
        $crate::class_variants!(@enum $size { $($sizes)* });

        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        pub struct $name { pub variant: $variant, pub size: $size }

        impl $crate::cn::class_variants::IntoTailwindClass for $name {
            fn raw_class(&self) -> ::std::borrow::Cow<'_, str> {
                ::std::borrow::Cow::Owned($crate::cn::join_text(&[
                    $base, self.variant.as_class(), self.size.as_class(),
                ]))
            }
        }
    };
    (
        $name:ident {
            base: $base:literal,
            variant: $variant:ident { $($variants:tt)* }
        }
    ) => {
        $crate::class_variants!(@enum $variant { $($variants)* });

        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        pub struct $name { pub variant: $variant }

        impl $crate::cn::class_variants::IntoTailwindClass for $name {
            fn raw_class(&self) -> ::std::borrow::Cow<'_, str> {
                ::std::borrow::Cow::Owned($crate::cn::join_text(&[$base, self.variant.as_class()]))
            }
        }
    };
}

pub use crate::class_variants;

#[cfg(test)]
mod tests {
    use super::IntoTailwindClass;

    crate::class_variants! {
        BadgeClass {
            base: "inline-flex p-2 bg-card",
            variant: BadgeVariant { Primary: "bg-primary", Outline: "border" },
            size: BadgeSize { Regular: "p-4", Compact: "p-1" }
        }
    }
    crate::class_variants! {
        VariantOnlyClass {
            base: "block",
            variant: OnlyVariant { First: "p-1", Second: "p-2" }
        }
    }

    #[test]
    fn first_arms_are_defaults_and_raw_fragments_are_retained() {
        let value = BadgeClass::default();
        assert_eq!(value.variant, BadgeVariant::Primary);
        assert_eq!(value.size, BadgeSize::Regular);
        assert_eq!(value.raw_class(), "inline-flex p-2 bg-card bg-primary p-4");
        assert_eq!(
            value.with_class("p-6 bg-secondary"),
            "inline-flex p-6 bg-secondary"
        );
        assert_eq!(value.to_class(), "inline-flex bg-primary p-4");
    }

    #[test]
    fn explicit_variant_and_size_preserve_order() {
        let value = BadgeClass {
            variant: BadgeVariant::Outline,
            size: BadgeSize::Compact,
        };
        assert_eq!(value.to_class(), "inline-flex bg-card border p-1");
    }

    #[test]
    fn variant_only_classes_and_enums_use_the_same_trait() {
        assert_eq!(VariantOnlyClass::default().to_class(), "block p-1");
        assert_eq!(OnlyVariant::Second.with_class("p-3"), "p-3");
    }
}
