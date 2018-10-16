@PHONY: nginx-configure nginx-module

NGX_RUST_NGINX_VERSION := 1.13.7
NGX_RUST_UNAME_S := $(shell uname -s)
NGX_RUST_MODULES := --with-compat  --with-threads --with-http_addition_module \
     --with-http_auth_request_module   --with-http_gunzip_module --with-http_gzip_static_module  \
     --with-http_random_index_module --with-http_realip_module --with-http_secure_link_module \
     --with-http_slice_module  --with-http_stub_status_module --with-http_sub_module \
     --with-stream --with-stream_realip_module --with-stream_ssl_preread_module

ifeq ($(NGX_RUST_UNAME_S),Linux)
	NGX_RUST_LD_OPT := -Wl,-Bsymbolic-functions -Wl,-z,relro -Wl,-z,now -Wl,--as-needed -pie
	NGX_RUST_CC_OPT := -g -fstack-protector-strong -Wformat -Werror=format-security -Wp,-D_FORTIFY_SOURCE=2 -fPIC
	NGX_RUST_LINUX_MODULES := --with-file-aio --with-http_ssl_module --with-stream_ssl_module
    NGX_RUST_PLATFORM_DIR := nginx-linux
    NGX_RUST_CONFIGURE_OPT := $(NGX_RUST_MODULES) \
		$(NGX_RUST_LINUX_MODULES) \
       --with-cc-opt='$(NGX_RUST_CC_OPT)' \
       --with-ld-opt='$(NGX_RUST_LD_OPT)'
endif
ifeq ($(NGX_RUST_UNAME_S),Darwin)
    NGX_RUST_PLATFORM_DIR := nginx-darwin
    NGX_RUST_CONFIGURE_OPT= $(NGX_RUST_MODULES)
endif

NGX_RUST_SRC_DIR := nginx/$(NGX_RUST_PLATFORM_DIR)


nginx :
	mkdir -p nginx

$(NGX_RUST_SRC_DIR) : nginx
	rm -rf $(NGX_RUST_SRC_DIR)
	wget http://nginx.org/download/nginx-$(NGX_RUST_NGINX_VERSION).tar.gz
	tar zxf nginx-$(NGX_RUST_NGINX_VERSION).tar.gz
	mv nginx-$(NGX_RUST_NGINX_VERSION) $(NGX_RUST_SRC_DIR)
	rm nginx-$(NGX_RUST_NGINX_VERSION).tar.gz*

$(NGX_RUST_SRC_DIR)/Makefile : $(NGX_RUST_SRC_DIR)
	cd $(NGX_RUST_SRC_DIR) && ./configure $(NGX_RUST_CONFIGURE_OPT)


nginx-configure : $(NGX_RUST_SRC_DIR)/Makefile


nginx-module : $(NGX_RUST_SRC_DIR)/Makefile
	$(MAKE) -C $(NGX_RUST_SRC_DIR) modules


clean:
	rm -rf nginx
	rm -f src/bindings.rs
