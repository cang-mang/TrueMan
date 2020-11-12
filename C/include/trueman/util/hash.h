/*
 * encoding=utf-8
 * 散列操作接口。
 * 历史：
 *     2020-11-12，完成Jenkins-HASH散列操作。
 */
#if !defined(TrueMan_UTIL_HASH)
    #define TrueMan_UTIL_HASH
    /*########################################################################*/
    #include <trueman/stddef.h>
    /*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
    TrueMan_extern
    TrueMan_U32_t TrueMan_UTIL_HASH_JENKINS_x_n(
        TrueMan_U32_t   const key[],
        TrueMan_USIZE_t       cnt  ,
        TrueMan_U32_t         seed
    );
    /*------------------------------------------------------------------------*/
    TrueMan_extern
    TrueMan_U32_t TrueMan_UTIL_HASH_JENKINS_x_3(
        TrueMan_U32_t a   ,
        TrueMan_U32_t b   ,
        TrueMan_U32_t c   ,
        TrueMan_U32_t seed
    );
    /*------------------------------------------------------------------------*/
    TrueMan_extern
    TrueMan_U32_t TrueMan_UTIL_HASH_JENKINS_x_2(
        TrueMan_U32_t a   ,
        TrueMan_U32_t b   ,
        TrueMan_U32_t seed
    );
    /*------------------------------------------------------------------------*/
    TrueMan_extern
    TrueMan_U32_t TrueMan_UTIL_HASH_JENKINS_x_1(
        TrueMan_U32_t a   ,
        TrueMan_U32_t seed
    );
    /*------------------------------------------------------------------------*/
    TrueMan_extern
    TrueMan_U32_t TrueMan_UTIL_HASH_JENKINS_x_0(
        void            const *ptr ,
        TrueMan_USIZE_t        len ,
        TrueMan_U32_t          seed
    );
    /*------------------------------------------------------------------------*/
    TrueMan_extern
    TrueMan_U64_t TrueMan_UTIL_HASH_WANG_direct(
        TrueMan_U64_t key
    );
    /*------------------------------------------------------------------------*/
    TrueMan_extern
    TrueMan_U64_t TrueMan_UTIL_HASH_WANG_inverse(
        TrueMan_U64_t key
    );
    /*------------------------------------------------------------------------*/
    TrueMan_extern
    TrueMan_U32_t TrueMan_UTIL_HASH_BKDR_x_0(
        void            const *ptr  ,
        TrueMan_USIZE_t        len  ,
        TrueMan_U32_t          seed ,
        TrueMan_U32_t          magic
    );
    /*------------------------------------------------------------------------*/
    TrueMan_extern
    TrueMan_U32_t TrueMan_UTIL_HASH_BKDR_time33(
        void            const *ptr  ,
        TrueMan_USIZE_t        len  ,
        TrueMan_U32_t          magic
    );
    /*------------------------------------------------------------------------*/
    TrueMan_extern
    TrueMan_U32_t TrueMan_UTIL_HASH_FNV1_x32(
        void            const *ptr,
        TrueMan_USIZE_t        len
    );
    /*------------------------------------------------------------------------*/
    TrueMan_extern
    TrueMan_U64_t TrueMan_UTIL_HASH_FNV1_x64(
        void            const *ptr,
        TrueMan_USIZE_t        len
    );
    /*------------------------------------------------------------------------*/
    TrueMan_extern
    TrueMan_U32_t TrueMan_UTIL_HASH_FNV1A_x32(
        void            const *ptr,
        TrueMan_USIZE_t        len
    );
    /*------------------------------------------------------------------------*/
    TrueMan_extern
    TrueMan_U64_t TrueMan_UTIL_HASH_FNV1A_x64(
        void            const *ptr,
        TrueMan_USIZE_t        len
    );
    /*------------------------------------------------------------------------*/
    TrueMan_extern
    TrueMan_U32_t TrueMan_UTIL_HASH_XX_x32(
        void            const *ptr ,
        TrueMan_USIZE_t        len ,
        TrueMan_U32_t          seed
    );
    /*------------------------------------------------------------------------*/
    TrueMan_extern
    TrueMan_U64_t TrueMan_UTIL_HASH_XX_x64(
        void            const *ptr ,
        TrueMan_USIZE_t        len ,
        TrueMan_U64_t          seed
    );
    /*########################################################################*/
    #if defined(__cplusplus)
        /*####################################################################*/
        namespace TrueMan { namespace util { namespace hash {
            /*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
            namespace jenkins {
                /*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
                static inline std::uint32_t x_n(
                    std::uint32_t  const key[],
                    std::uintptr_t       cnt  ,
                    std::uint32_t        seed
                ) {
                    return(TrueMan_UTIL_HASH_JENKINS_x_n(key, cnt, seed));
                }
                /*------------------------------------------------------------*/
                static inline std::uint32_t x_3(
                    std::uint32_t a   ,
                    std::uint32_t b   ,
                    std::uint32_t c   ,
                    std::uint32_t seed
                ) {
                    return(TrueMan_UTIL_HASH_JENKINS_x_3(a, b, c, seed));
                }
                /*------------------------------------------------------------*/
                static inline std::uint32_t x_2(
                    std::uint32_t a   ,
                    std::uint32_t b   ,
                    std::uint32_t seed
                ) {
                    return(TrueMan_UTIL_HASH_JENKINS_x_2(a, b, seed));
                }
                /*------------------------------------------------------------*/
                static inline std::uint32_t x_1(
                    std::uint32_t a   ,
                    std::uint32_t seed
                ) {
                    return(TrueMan_UTIL_HASH_JENKINS_x_1(a, seed));
                }
                /*------------------------------------------------------------*/
                static inline std::uint32_t x_0(
                    void           const *ptr ,
                    std::uintptr_t        len ,
                    std::uint32_t         seed
                ) {
                    return(TrueMan_UTIL_HASH_JENKINS_x_0(ptr, len, seed));
                }
                /*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
            }
            /*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
            namespace wang {
                static inline std::uint64_t direct(
                    std::uint64_t key
                ) {
                    return(TrueMan_UTIL_HASH_WANG_direct(key));
                }
                /*------------------------------------------------------------*/
                static inline std::uint64_t inverse(
                    std::uint64_t key
                ) {
                    return(TrueMan_UTIL_HASH_WANG_inverse(key));
                }
            }
            /*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
            namespace bkdr {
                static inline std::uint32_t x_0(
                    void           const *ptr  ,
                    std::uintptr_t        len  ,
                    std::uint32_t         seed ,
                    std::uint32_t         magic
                ) {
                    return(TrueMan_UTIL_HASH_BKDR_x_0(ptr, len, seed, magic));
                }
                /*------------------------------------------------------------*/
                static inline std::uint32_t time33(
                    void           const *ptr  ,
                    std::uintptr_t        len  ,
                    std::uint32_t         magic
                ) {
                    return(TrueMan_UTIL_HASH_BKDR_time33(ptr, len, magic));
                }
            }
            /*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
            namespace fnv1 {
                static inline std::uint32_t x32(
                    void           const *ptr,
                    std::uintptr_t        len
                ) {
                    return(TrueMan_UTIL_HASH_FNV1_x32(ptr, len));
                }
                /*------------------------------------------------------------*/
                static inline std::uint64_t x64(
                    void           const *ptr,
                    std::uintptr_t        len
                ) {
                    return(TrueMan_UTIL_HASH_FNV1_x64(ptr, len));
                }
            }
            /*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
            namespace fnv1a {
                static inline std::uint32_t x32(
                    void           const *ptr,
                    std::uintptr_t        len
                ) {
                    return(TrueMan_UTIL_HASH_FNV1A_x32(ptr, len));
                }
                /*------------------------------------------------------------*/
                static inline std::uint64_t x64(
                    void           const *ptr,
                    std::uintptr_t        len
                ) {
                    return(TrueMan_UTIL_HASH_FNV1A_x64(ptr, len));
                }
            }
            /*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
            namespace xx {
                static inline std::uint32_t x32(
                    void           const *ptr ,
                    std::uintptr_t        len ,
                    std::uint32_t         seed
                ) {
                    return(TrueMan_UTIL_HASH_XX_x32(ptr, len, seed));
                }
                /*------------------------------------------------------------*/
                static inline std::uint64_t x64(
                    void           const *ptr ,
                    std::uintptr_t        len ,
                    std::uint64_t         seed
                ) {
                    return(TrueMan_UTIL_HASH_XX_x64(ptr, len, seed));
                }
            }
            /*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
        } } }
        /*####################################################################*/
    #endif
    /*########################################################################*/
#endif
