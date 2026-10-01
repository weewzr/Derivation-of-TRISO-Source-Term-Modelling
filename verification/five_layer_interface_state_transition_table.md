# Five-Layer Eight-State Transition Table

Documentation/audit support only. The hard matrix is not implemented here.

| Source | Region exit | Interface | Outcome | Destination | Transform factor |
|---|---|---|---|---|---|
| S0 Kernel@I0 | outer | I0 | reflect Kernel | S0 | H_K p_KK |
| S0 Kernel@I0 | outer | I0 | transmit Buffer | S1 | H_K p_KB |
| S1 Buffer@I0 | inner | I0 | transmit Kernel | S0 | G_B^- p_BK |
| S1 Buffer@I0 | inner | I0 | reflect Buffer | S1 | G_B^- p_BB(I0) |
| S1 Buffer@I0 | outer | I1 | reflect Buffer | S2 | G_B^+ p_BB(I1) |
| S1 Buffer@I0 | outer | I1 | transmit IPyC | S3 | G_B^+ p_BI |
| S2 Buffer@I1 | inner | I0 | transmit Kernel | S0 | G_B^- p_BK |
| S2 Buffer@I1 | inner | I0 | reflect Buffer | S1 | G_B^- p_BB(I0) |
| S2 Buffer@I1 | outer | I1 | reflect Buffer | S2 | G_B^+ p_BB(I1) |
| S2 Buffer@I1 | outer | I1 | transmit IPyC | S3 | G_B^+ p_BI |
| S3 IPyC@I1 | inner | I1 | transmit Buffer | S2 | G_I^- p_IB |
| S3 IPyC@I1 | inner | I1 | reflect IPyC | S3 | G_I^- p_II(I1) |
| S3 IPyC@I1 | outer | I2 | reflect IPyC | S4 | G_I^+ p_II(I2) |
| S3 IPyC@I1 | outer | I2 | transmit SiC | S5 | G_I^+ p_IS |
| S4 IPyC@I2 | inner | I1 | transmit Buffer | S2 | G_I^- p_IB |
| S4 IPyC@I2 | inner | I1 | reflect IPyC | S3 | G_I^- p_II(I1) |
| S4 IPyC@I2 | outer | I2 | reflect IPyC | S4 | G_I^+ p_II(I2) |
| S4 IPyC@I2 | outer | I2 | transmit SiC | S5 | G_I^+ p_IS |
| S5 SiC@I2 | inner | I2 | transmit IPyC | S4 | G_S^- p_SI |
| S5 SiC@I2 | inner | I2 | reflect SiC | S5 | G_S^- p_SS(I2) |
| S5 SiC@I2 | outer | I3 | reflect SiC | S6 | G_S^+ p_SS(I3) |
| S5 SiC@I2 | outer | I3 | transmit OPyC | S7 | G_S^+ p_SO |
| S6 SiC@I3 | inner | I2 | transmit IPyC | S4 | G_S^- p_SI |
| S6 SiC@I3 | inner | I2 | reflect SiC | S5 | G_S^- p_SS(I2) |
| S6 SiC@I3 | outer | I3 | reflect SiC | S6 | G_S^+ p_SS(I3) |
| S6 SiC@I3 | outer | I3 | transmit OPyC | S7 | G_S^+ p_SO |
| S7 OPyC@I3 | inner | I3 | transmit SiC | S6 | G_O^- p_OS |
| S7 OPyC@I3 | inner | I3 | reflect OPyC | S7 | G_O^- p_OO |
| S7 OPyC@I3 | outer | R | absorb | Released | G_O^+ |

Each G is evaluated at the deterministic reinsertion radius of the source state. This table describes Process B/C exact-interface renewal, not Process A finite-capture production WOS.
