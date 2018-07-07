#include "dpdk-inlined.h"

struct rte_mbuf *rte_pktmbuf_alloc1(struct rte_mempool *mp)
{
        struct rte_mbuf *m;
        if ((m = rte_mbuf_raw_alloc(mp)) != NULL)
                rte_pktmbuf_reset(m);
        return m;
}

char *rte_pktmbuf_append1(struct rte_mbuf *m, uint16_t len)
{
        void *tail;
        struct rte_mbuf *m_last;

        __rte_mbuf_sanity_check(m, 1);

        m_last = rte_pktmbuf_lastseg(m);
        if (unlikely(len > rte_pktmbuf_tailroom(m_last)))
                return NULL;

        tail = (char *)m_last->buf_addr + m_last->data_off + m_last->data_len;
        m_last->data_len = (uint16_t)(m_last->data_len + len);
        m->pkt_len = (m->pkt_len + len);
        return (char *)tail;
}