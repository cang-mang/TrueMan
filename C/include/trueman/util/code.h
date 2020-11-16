/*
 * encoding=utf-8
 * 散列操作接口。
 * 历史：
 *     2020-11-13，完成BASE16编解码操作。
 */
#if !defined(TrueMan_UTIL_CODE)
    #define TrueMan_UTIL_CODE
    /*########################################################################*/
    #include <trueman/stddef.h>
    /*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
    //1字节的数据，编码后得到2字节。
    //txt的字节数至少是len乘以二，所以txt的字节数至少为：
    //    len * 2
    //用位操作加速可写为：
    //    len << 1
    //cap表示字母是否用大写形式输出。
    //返回实际写入txt的字节数。
    TrueMan_extern
    TrueMan_USIZE_t TrueMan_UTIL_CODE_BASE16_encode(
        TrueMan_U8_t    const raw[],
        TrueMan_USIZE_t       len  ,
        TrueMan_U8_t          txt[],
        TrueMan_I8_t          cap
    );
    /*------------------------------------------------------------------------*/
    //2字节的数据，解码后得到1字节。
    //len应是2的整数倍；raw的字节数至少为：
    //    len / 2
    //用位操作加速可写为：
    //    len >> 1
    //返回0或正数，表示成功，写入raw多少字节；
    //返回负数，表示失败，其相反数减1，表示写入raw多少字节。
    TrueMan_extern
    TrueMan_ISIZE_t TrueMan_UTIL_CODE_BASE16_decode(
        TrueMan_U8_t const txt[], TrueMan_USIZE_t len, TrueMan_U8_t raw[]
    );
    /*########################################################################*/
    #if defined(__cplusplus)
        /*####################################################################*/
        namespace TrueMan { namespace util { namespace code {
            /*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
            namespace base16 {
                /*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
                static inline
                std::uintptr_t encode(
                    std::uint8_t   const raw[],
                    std::uintptr_t       len  ,
                    std::uint8_t         txt[],
                    bool                 cap
                ) {
                    return(TrueMan_UTIL_CODE_BASE16_encode(
                        raw, len, txt, static_cast<std::int8_t>(cap)
                    ));
                }
                /*------------------------------------------------------------*/
                static inline
                std::intptr_t decode(
                    std::uint8_t   const txt[],
                    std::uintptr_t       len  ,
                    std::uint8_t         raw[]
                ) {
                    return(TrueMan_UTIL_CODE_BASE16_decode(txt, len, raw));
                }
                /*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
            }
            /*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
        } } }
        /*####################################################################*/
    #endif
    /*########################################################################*/
#endif
