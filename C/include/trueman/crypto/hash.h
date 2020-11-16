/*
 * encoding=utf-8
 * 摘要操作接口。
 * 历史：
 *     2020-11-12，完成FNV128a-HASH算法操作。
 *     2020-11-13，完成网络报文校验和算法算法操作。
 */
#if !defined(TrueMan_CRYPTO_HASH)
    #define TrueMan_CRYPTO_HASH
    /*########################################################################*/
    #include <trueman/stddef.h>
    /*||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
    typedef struct TrueMan_CRYPTO_HASH_op {
        void *(*init)  (void *arg                                      );
        void *(*update)(void *arg, void const *ptr, TrueMan_USIZE_t len);
        void *(*final) (void *arg, TrueMan_U8_t *val                   );
        /*--------------------------------------------------------------------*/
        void (*one)(void const *ptr, TrueMan_USIZE_t len, TrueMan_U8_t *val);
        /*--------------------------------------------------------------------*/
        TrueMan_USIZE_t len_ctx;
        TrueMan_USIZE_t len_blk;
        TrueMan_USIZE_t len_dgt;
    } TrueMan_CRYPTO_HASH_OP_t;
    /*========================================================================*/
    #define TrueMan_CRYPTO_HASH_FNV128A_len_blk 1
    #define TrueMan_CRYPTO_HASH_FNV128A_len_dgt 12
    /*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/
    typedef struct TrueMan_CRYPTO_HASH_FNV128A_ctx {
        TrueMan_U8_t val[16];
    } TrueMan_CRYPTO_HASH_FNV128A_CTX_t;
    /*========================================================================*/
    TrueMan_extern
    TrueMan_CRYPTO_HASH_OP_t const TrueMan_CRYPTO_HASH_FNV128A_op;
    /*------------------------------------------------------------------------*/
    static inline
    TrueMan_CRYPTO_HASH_FNV128A_CTX_t *TrueMan_CRYPTO_HASH_FNV128A_init(
        TrueMan_CRYPTO_HASH_FNV128A_CTX_t *ctx
    ) {
        return((
            TrueMan_CRYPTO_HASH_FNV128A_CTX_t *
        )TrueMan_CRYPTO_HASH_FNV128A_op.init(ctx));
    }
    /*------------------------------------------------------------------------*/
    static inline
    TrueMan_CRYPTO_HASH_FNV128A_CTX_t *TrueMan_CRYPTO_HASH_FNV128A_update(
        TrueMan_CRYPTO_HASH_FNV128A_CTX_t       *ctx,
        void                              const *ptr,
        TrueMan_USIZE_t                          len
    ) {
        return((
            TrueMan_CRYPTO_HASH_FNV128A_CTX_t *
        )TrueMan_CRYPTO_HASH_FNV128A_op.update(ctx, ptr, len));
    }
    /*------------------------------------------------------------------------*/
    static inline
    TrueMan_CRYPTO_HASH_FNV128A_CTX_t *TrueMan_CRYPTO_HASH_FNV128A_final(
        TrueMan_CRYPTO_HASH_FNV128A_CTX_t *ctx,
        TrueMan_U8_t                       val[
            TrueMan_CRYPTO_HASH_FNV128A_len_dgt
        ]
    ) {
        return((
            TrueMan_CRYPTO_HASH_FNV128A_CTX_t *
        )TrueMan_CRYPTO_HASH_FNV128A_op.final(ctx, val));
    }
    /*------------------------------------------------------------------------*/
    static inline void TrueMan_CRYPTO_HASH_FNV128A_one(
        void            const *ptr                                     ,
        TrueMan_USIZE_t        len                                     ,
        TrueMan_U8_t           val[TrueMan_CRYPTO_HASH_FNV128A_len_dgt]
    ) {
        TrueMan_CRYPTO_HASH_FNV128A_op.one(ptr, len, val);
    }
    /*------------------------------------------------------------------------*/
    #define TrueMan_CRYPTO_HASH_CSUM_len_blk 2
    #define TrueMan_CRYPTO_HASH_CSUM_len_dgt 2
    /*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/
    typedef struct TrueMan_CRYPTO_HASH_CSUM_ctx {
        union {
            TrueMan_U8_t  b[2];
            TrueMan_U16_t s;
        }             bak;
        TrueMan_U32_t sum;
    } TrueMan_CRYPTO_HASH_CSUM_CTX_t;
    /*========================================================================*/
    TrueMan_extern
    TrueMan_CRYPTO_HASH_OP_t const TrueMan_CRYPTO_HASH_CSUM_op;
    /*------------------------------------------------------------------------*/
    static inline
    TrueMan_CRYPTO_HASH_CSUM_CTX_t *TrueMan_CRYPTO_HASH_CSUM_init(
        TrueMan_CRYPTO_HASH_CSUM_CTX_t *ctx
    ) {
        return((
            TrueMan_CRYPTO_HASH_CSUM_CTX_t *
        )TrueMan_CRYPTO_HASH_CSUM_op.init(ctx));
    }
    /*------------------------------------------------------------------------*/
    static inline
    TrueMan_CRYPTO_HASH_CSUM_CTX_t *TrueMan_CRYPTO_HASH_CSUM_update(
        TrueMan_CRYPTO_HASH_CSUM_CTX_t       *ctx,
        void                           const *ptr,
        TrueMan_USIZE_t                       len
    ) {
        return((
            TrueMan_CRYPTO_HASH_CSUM_CTX_t *
        )TrueMan_CRYPTO_HASH_CSUM_op.update(ctx, ptr, len));
    }
    /*------------------------------------------------------------------------*/
    static inline
    TrueMan_CRYPTO_HASH_CSUM_CTX_t *TrueMan_CRYPTO_HASH_CSUM_final(
        TrueMan_CRYPTO_HASH_CSUM_CTX_t *ctx                                  ,
        TrueMan_U8_t                    val[TrueMan_CRYPTO_HASH_CSUM_len_dgt]
    ) {
        return((
            TrueMan_CRYPTO_HASH_CSUM_CTX_t *
        )TrueMan_CRYPTO_HASH_CSUM_op.final(ctx, val));
    }
    /*------------------------------------------------------------------------*/
    static inline void TrueMan_CRYPTO_HASH_CSUM_one(
        void            const *ptr                                  ,
        TrueMan_USIZE_t        len                                  ,
        TrueMan_U8_t           val[TrueMan_CRYPTO_HASH_CSUM_len_dgt]
    ) {
        TrueMan_CRYPTO_HASH_CSUM_op.one(ptr, len, val);
    }
    /*########################################################################*/
    #if defined(__cplusplus)
        /*####################################################################*/
        namespace TrueMan { namespace crypto { namespace hash {
            /*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
            class Op {
                Op(Op const  &) = delete;
                Op(Op       &&) = delete;
                /*------------------------------------------------------------*/
                Op &operator=(Op const  &) = delete;
                Op &operator=(Op       &&) = delete;
                /*------------------------------------------------------------*/
                protected:
                    inline
                    Op(TrueMan_CRYPTO_HASH_OP_t const &op, void *arg): op_(op) {
                        op.init(this + 1);
                    }
                    /*--------------------------------------------------------*/
                public:
                    inline
                    Op &init() {
                        op_.init(this + 1);
                        return(*this);
                    }
                    /*--------------------------------------------------------*/
                    inline
                    Op &update(void const *ptr, std::uintptr_t len) {
                        op_.update(this + 1, ptr, len);
                        return(*this);
                    }
                    /*--------------------------------------------------------*/
                    inline
                    Op &final(std::uint8_t val[]) {
                        op_.final(this + 1, val);
                        return(*this);
                    }
                    /*--------------------------------------------------------*/
                public:
                    inline
                    void one(
                        void const *ptr, std::uintptr_t len, std::uint8_t val[]
                    ) {
                        op_.one(ptr, len, val);
                    }
                    /*--------------------------------------------------------*/
                public:
                    inline
                    std::uintptr_t len_blk() const {
                        return(op_.len_blk);
                    }
                    /*--------------------------------------------------------*/
                    inline
                    std::uintptr_t len_dgt() const {
                        return(op_.len_dgt);
                    }
                    /*--------------------------------------------------------*/
                private:
                    TrueMan_CRYPTO_HASH_OP_t const &op_;
            };
            /*================================================================*/
            namespace fnv128a {
                /*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
                constexpr std::uintptr_t len_blk =
                    TrueMan_CRYPTO_HASH_FNV128A_len_blk
                ;
                constexpr std::uintptr_t len_dgt =
                    TrueMan_CRYPTO_HASH_FNV128A_len_dgt
                ;
                /*------------------------------------------------------------*/
                constexpr TrueMan_CRYPTO_HASH_OP_t const &op =
                    TrueMan_CRYPTO_HASH_FNV128A_op
                ;
                /*------------------------------------------------------------*/
                class Ctx: public Op {
                    public:
                        inline
                        Ctx(): Op(op, &ctx_) {
                        }
                        /*----------------------------------------------------*/
                    private:
                        TrueMan_CRYPTO_HASH_FNV128A_CTX_t ctx_;
                };
                /*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
            }
            /*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
            namespace csum {
                /*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
                constexpr std::uintptr_t len_blk =
                    TrueMan_CRYPTO_HASH_CSUM_len_blk
                ;
                constexpr std::uintptr_t len_dgt =
                    TrueMan_CRYPTO_HASH_CSUM_len_dgt
                ;
                /*------------------------------------------------------------*/
                constexpr TrueMan_CRYPTO_HASH_OP_t const &op =
                    TrueMan_CRYPTO_HASH_CSUM_op
                ;
                /*------------------------------------------------------------*/
                class Ctx: public Op {
                    public:
                        inline
                        Ctx(): Op(op, &ctx_) {
                        }
                        /*----------------------------------------------------*/
                    private:
                        TrueMan_CRYPTO_HASH_CSUM_CTX_t ctx_;
                };
                /*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
            }
            /*::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::*/
        } } }
        /*####################################################################*/
    #endif
    /*########################################################################*/
#endif
