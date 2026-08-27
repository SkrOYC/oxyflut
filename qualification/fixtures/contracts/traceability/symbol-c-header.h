typedef struct Owner {
  int other;
} Owner;

typedef struct DifferentOwner {
  void (*member)(void);
} DifferentOwner;
