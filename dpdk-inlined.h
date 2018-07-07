#include "rte.h"

struct rte_mbuf *rte_pktmbuf_alloc1(struct rte_mempool *mp);
char *rte_pktmbuf_append1(struct rte_mbuf *m, uint16_t len);
