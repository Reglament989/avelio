typedef struct Buffer {
  uint8_t *data;
  uintptr_t len;
} Buffer;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void init(const char *base_url_ptr, const char *token_ptr);

const char *sign_in(const char *login_ptr, const char *password_ptr);

const char *sign_up(const char *login_ptr, const char *password_ptr);

const char *refresh_token(const char *token_ptr);

const char *tracks(int64_t limit, int64_t offset);

const char *track_by_id(const char *id_ptr);

struct Buffer get_bytes_of_track(const char *id_ptr);

const char *upload_track(struct Buffer bytes_buf, const char *song_ptr);

char *tr(const char *key_translation, uintptr_t size_args, const char *array_pointer);

void free_char(const char *buf);

void free_buf(struct Buffer buf);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
