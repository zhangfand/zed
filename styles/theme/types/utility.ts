/**
 * Utility type to improve the readability of types.
 *
 * Wrapping a type or interface in `Prettify` will make it so that the type or interface properties are properly displayed when hovering over them.
 */
export type Prettify<T> = {
  [K in keyof T]: T[K]
}

/**
 * `DeepMerge` allows for deep merging of two objects, including nested properties. The resulting object will contain all properties from both input objects.
 */
type MergeToOne<T> = (
  T extends object ? { [K in keyof T]: (
    K extends RequiredKeys<T> ? Exclude<T[K], undefined> : T[K]
  ) } : never
)

type RequiredKeys<T> = { [K in keyof T]-?: object extends Pick<T, K> ? never : K }[keyof T];

type OptionalKeys<T> = { [K in keyof T]-?: object extends Pick<T, K> ? K : never }[keyof T];

export type DeepMerge<T1, T2> = (
  T1 extends object ? (
    T2 extends object ? (
      MergeToOne<(
        { [K in (keyof T2 & keyof T1 & RequiredKeys<T1 | T2>)]: DeepMerge<T1[K], T2[K]> }
        & { [K in (keyof T2 & keyof T1 & OptionalKeys<T1 | T2>)]?: DeepMerge<T1[K], T2[K]> }

        & { [K in Exclude<RequiredKeys<T1>, keyof T2>]: T1[K] }
        & { [K in Exclude<OptionalKeys<T1>, keyof T2>]?: T1[K] }

        & { [K in Exclude<RequiredKeys<T2>, keyof T1>]: T2[K] }
        & { [K in Exclude<OptionalKeys<T2>, keyof T1>]?: T2[K] }
      )>
    ) : (
      T1 extends object ? T2 : T1 | T2
    )
  ) : (
    T2 extends object ? T1 : T1 | T2
  )
);

/**
 * `DeepRequire` allows for deep requiring of an object's properties. The resulting object will require all properties from the input object.
 */
export type DeepRequire<T> = {
  [P in keyof T]-?: T[P] extends object ? DeepRequire<T[P]> : T[P];
};

/**
 * `DeepPartial` allows for deep partializing of an object, making all its properties optional, including nested properties.
 */
export type DeepPartial<T> = {
  [P in keyof T]?: T[P] extends object ? DeepPartial<T[P]> : T[P];
};
