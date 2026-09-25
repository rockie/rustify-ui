/// Declares a component whose classes depend on a variant and a size.
///
/// It generates the class struct, the two enums (the first arm of each is the
/// default), and optionally the component itself. Callers set `variant` and
/// `size` as values, not as class strings, so the set of looks a component has
/// is a type rather than a convention.
///
/// ```ignore
/// variants! {
///     Badge {
///         base: "inline-flex items-center rounded-md",
///         variants: {
///             variant: {
///                 Default: "bg-primary text-primary-foreground",
///                 Outline: "border text-foreground",
///             },
///             size: {
///                 Default: "px-2.5 py-0.5 text-xs",
///                 Lg: "px-3 py-1 text-sm",
///             }
///         },
///         component: { element: span }
///     }
/// }
/// ```
///
/// The `link: true` form renders an `<a>` and marks it `aria-current="page"`
/// while the current path is at or below its `href`. What "the current path"
/// is comes from [`crate::current_path`] - this crate has no router and does
/// not want one; the SDK's router provides that signal, and without one a link
/// is simply never current.
#[macro_export]
macro_rules! variants {
    // Variant + size, with a component.
    (
        $component:ident {
            base: $base_class:literal,
            variants: {
                variant: {
                    $first_variant:ident: $first_variant_class:literal
                    $(, $variant_key:ident: $variant_class:literal)* $(,)?
                },
                size: {
                    $first_size:ident: $first_size_class:literal
                    $(, $size_key:ident: $size_class:literal)* $(,)?
                }
            },
            component: {
                element: $element:ident
            }
        }
    ) => {
        $crate::variants! {
            $component {
                base: $base_class,
                variants: {
                    variant: { $first_variant: $first_variant_class $(, $variant_key: $variant_class)* },
                    size: { $first_size: $first_size_class $(, $size_key: $size_class)* }
                }
            }
        }

        $crate::paste::paste! {
            #[::leptos::component]
            pub fn $component(
                #[prop(into, optional)] variant: ::leptos::prelude::Signal<[<$component Variant>]>,
                #[prop(into, optional)] size: ::leptos::prelude::Signal<[<$component Size>]>,
                #[prop(into, optional)] class: ::leptos::prelude::Signal<String>,
                #[prop(into, optional)] test_id: String,
                children: ::leptos::prelude::Children,
            ) -> impl ::leptos::prelude::IntoView {
                use ::leptos::prelude::*;

                let computed = move || {
                    [<$component Class>] {
                        variant: variant.try_get().unwrap_or_default(),
                        size: size.try_get().unwrap_or_default(),
                    }
                    .with_class(class.try_get().unwrap_or_default())
                };

                ::leptos::view! {
                    <$element
                        class=computed
                        data-name=stringify!($component)
                        data-testid=test_id
                    >
                        {children()}
                    </$element>
                }
            }
        }
    };

    // Variant + size, with a component that is a link.
    (
        $component:ident {
            base: $base_class:literal,
            variants: {
                variant: {
                    $first_variant:ident: $first_variant_class:literal
                    $(, $variant_key:ident: $variant_class:literal)* $(,)?
                },
                size: {
                    $first_size:ident: $first_size_class:literal
                    $(, $size_key:ident: $size_class:literal)* $(,)?
                }
            },
            component: {
                element: $element:ident,
                link: true
            }
        }
    ) => {
        $crate::variants! {
            $component {
                base: $base_class,
                variants: {
                    variant: { $first_variant: $first_variant_class $(, $variant_key: $variant_class)* },
                    size: { $first_size: $first_size_class $(, $size_key: $size_class)* }
                }
            }
        }

        $crate::paste::paste! {
            #[::leptos::component]
            pub fn $component(
                #[prop(into, optional)] variant: ::leptos::prelude::Signal<[<$component Variant>]>,
                #[prop(into, optional)] size: ::leptos::prelude::Signal<[<$component Size>]>,
                #[prop(into, optional)] class: ::leptos::prelude::Signal<String>,
                #[prop(into, optional)] test_id: String,
                #[prop(into, optional)] href: Option<String>,
                children: ::leptos::prelude::Children,
            ) -> impl ::leptos::prelude::IntoView {
                use ::leptos::prelude::*;

                let computed = move || {
                    [<$component Class>] {
                        variant: variant.try_get().unwrap_or_default(),
                        size: size.try_get().unwrap_or_default(),
                    }
                    .with_class(class.try_get().unwrap_or_default())
                };

                match href {
                    Some(href) => {
                        let path = $crate::current_path();
                        let target = href.clone();
                        let current = move || {
                            match path.as_ref().map(|path| path.get()) {
                                Some(path) if $crate::is_within(&path, &target) => Some("page"),
                                _ => None,
                            }
                        };
                        ::leptos::view! {
                            <a
                                class=computed
                                href=href
                                aria-current=current
                                data-name=stringify!($component)
                                data-testid=test_id
                            >
                                {children()}
                            </a>
                        }
                        .into_any()
                    }
                    None => ::leptos::view! {
                        <$element
                            class=computed
                            data-name=stringify!($component)
                            data-testid=test_id
                        >
                            {children()}
                        </$element>
                    }
                    .into_any(),
                }
            }
        }
    };

    // Variant + size, types only.
    (
        $component:ident {
            base: $base_class:literal,
            variants: {
                variant: {
                    $first_variant:ident: $first_variant_class:literal
                    $(, $variant_key:ident: $variant_class:literal)* $(,)?
                },
                size: {
                    $first_size:ident: $first_size_class:literal
                    $(, $size_key:ident: $size_class:literal)* $(,)?
                }
            }
        }
    ) => {
        $crate::paste::paste! {
            // The glob is what the derives need: they generate code naming the
            // traits by their bare names. Whether every name in it is used
            // depends on which arm expanded, so the warning is not a finding.
            #[allow(unused_imports)]
            use $crate::tw_merge::*;

            #[derive(TwClass, Clone, Copy)]
            #[tw(class = $base_class)]
            pub struct [<$component Class>] {
                pub variant: [<$component Variant>],
                pub size: [<$component Size>],
            }

            #[derive(TwVariant)]
            pub enum [<$component Variant>] {
                #[tw(default, class = $first_variant_class)]
                $first_variant,
                $(#[tw(class = $variant_class)] $variant_key,)*
            }

            #[derive(TwVariant)]
            pub enum [<$component Size>] {
                #[tw(default, class = $first_size_class)]
                $first_size,
                $(#[tw(class = $size_class)] $size_key,)*
            }
        }
    };

    // Variant only, types only.
    (
        $component:ident {
            base: $base_class:literal,
            variants: {
                variant: {
                    $first_variant:ident: $first_variant_class:literal
                    $(, $variant_key:ident: $variant_class:literal)* $(,)?
                }
            }
        }
    ) => {
        $crate::paste::paste! {
            #[allow(unused_imports)]
            use $crate::tw_merge::*;

            #[derive(TwClass, Clone, Copy)]
            #[tw(class = $base_class)]
            pub struct [<$component Class>] {
                pub variant: [<$component Variant>],
            }

            #[derive(TwVariant)]
            pub enum [<$component Variant>] {
                #[tw(default, class = $first_variant_class)]
                $first_variant,
                $(#[tw(class = $variant_class)] $variant_key,)*
            }
        }
    };
}
