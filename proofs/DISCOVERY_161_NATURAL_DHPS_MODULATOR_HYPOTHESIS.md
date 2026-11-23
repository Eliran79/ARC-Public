# Discovery 161: A Candidate Natural DHPS-Targeting Agent in Brewer's Yeast and Button Mushroom — Hypothesis for Laboratory Testing

**Status: HYPOTHESIS. No experimental data exist. Nothing in this document is a claim of efficacy, safety or clinical benefit, and it is not medical advice.**

## Executive Summary

Field observation by the author (Ness Ziona offices) points to a candidate agent present in brewer's yeast (*Saccharomyces cerevisiae*) and in button mushrooms (*Agaricus bisporus*, champignon) that may act on the hypusine pathway of eIF5A through deoxyhypusine synthase (DHPS), the target proposed in Discovery 126. The proposed consequence is that lung cancer cells lose the ability to cope with free radicals (reactive oxygen species) and die, releasing the radicals into the cell and the body; the author adds that the body must then be supported nutritionally to cope with them.

This discovery records the hypothesis, the points where it is consistent with or in tension with existing data, and a laboratory plan that can confirm or falsify it.

## Hierarchical Position

```
Discovery 124 (Pharmacology):   bounded druggable target space
        ↓
Discovery 125 (SHMT2):          SGOC bottleneck hypothesis
        ↓
Discovery 126 (DHPS):           DHPS as backup-free dependency (Erratum, Nov 2026)
        ↓
Discovery 161 (this):           candidate natural DHPS-targeting agent -> TEST IN LAB
```

## Part 1: The Hypothesis (as stated by the author)

| Element | Statement |
|---------|-----------|
| Source | Brewer's yeast and button mushroom (field observation, not yet characterised) |
| Target | DHPS (hypusination of eIF5A) in lung cancer |
| Proposed effect | Cancer cell cannot overcome free radicals -> cell death; radicals released into cell/body |
| Supportive measure | Healthy diet to help the body cope with released radicals |

## Part 2: What Existing Data Say (from the D126 evaluation, Nov 2026)

**Consistent:**
- DHPS is essential in 89% of 1,258 cancer cell lines in DepMap (EIF5A 92%, DOHH 60%); there is no paralog, and the hypusine pathway is linear (DHPS and DOHH are cut vertices).
- Open Targets lists zero clinical candidates for DHPS.

**In tension / unresolved:**
1. **Spermidine is the DHPS substrate, not an inhibitor.** Yeast and mushrooms are rich in polyamines including spermidine; a substrate-rich food would be expected to supply the enzyme, not block it. The active agent, if any, must be something else (or act at another step) and has to be identified.
2. **DHPS is not upregulated in lung tumors** (median z vs. normal lung: LUAD -0.25, LUSC +0.14) and is broadly essential, so selectivity for cancer over normal cells cannot be assumed and must be measured.
3. **The ROS mechanism is not established.** The literature supports loss of hypusinated eIF5A causing ribosome stalling at polyproline motifs; a link to loss of ROS defence is a separate claim that is to be tested here.
4. **"Released radicals" and dietary support are untested** and are not evaluated in this document.

## Part 3: Laboratory Plan

| Step | Experiment | Positive control | Readout |
|------|------------|------------------|---------|
| 1 | Identify the active fraction: extract yeast/mushroom, fractionate, LC-MS characterisation | — | Fraction(s) with activity in steps 2–3; compound ID |
| 2 | **Cell-free DHPS assay**: recombinant human DHPS + eIF5A + spermidine; test fractions/compounds | GC7 (N1-guanyl-1,7-diaminoheptane) | Deoxyhypusine formation (e.g. labelled spermidine incorporation); IC50 |
| 3 | **Cellular target engagement**: lung cancer lines (e.g. A549, H1299) | GC7 | Hypusinated vs. total eIF5A (western); DHPS/DOHH levels |
| 4 | **Viability and selectivity**: cancer lines vs. normal bronchial epithelial cells (e.g. BEAS-2B) | GC7; DHFR/TYMS inhibitor as dependency control | Dose–response; selectivity index (normal IC50 / cancer IC50) |
| 5 | **ROS mechanism**: DCFDA / MitoSOX; rescue with N-acetylcysteine | — | ROS rise preceding death; rescue by antioxidant |
| 6 | **Spermidine control**: add spermidine alone and with the fraction | — | Does substrate supply increase hypusination or block the effect? |
| 7 | **Genetic specificity**: DHPS knockdown / rescue with resistant allele | — | Effect phenocopied by DHPS loss and rescued by DHPS |

## Part 4: Falsification Criteria

The hypothesis is rejected if any of the following holds:
1. No fraction inhibits recombinant DHPS or reduces hypusinated eIF5A in cells (steps 2–3).
2. Cell death occurs without reduced hypusination, i.e. the effect is not DHPS-mediated (steps 3, 7).
3. Selectivity index ≤ 1 (normal cells as sensitive as cancer cells) (step 4).
4. Cell death is not preceded by, or not rescued against, ROS increase (step 5).
5. Spermidine alone reproduces, or the active fraction is spermidine itself (step 6).

## Part 5: Why This Is a Bounded-Transformation Prediction

The hypusine pathway is a single linear chain (eIF5A -> deoxyhypusyl-eIF5A -> hypusyl-eIF5A) with no redundant route. A bounded system with no backup has a small number of ways to fail: interruption at DHPS or DOHH is predicted to produce a single dominant phenotype (loss of hypusinated eIF5A), which is why a laboratory test has a clear yes/no outcome. This follows the D126 erratum: the predictive variable is absence of a backup, not the substrate+product count.

## Part 6: Limitations

- The source material is an anecdotal field observation; the active component, dose and preparation are unknown.
- Food-derived mixtures contain many bioactive components; activity in a crude extract does not identify the cause.
- Broad DHPS essentiality may mean toxicity to normal cells; this is a central risk to the hypothesis.
- Nothing here supports use by patients. Anyone with lung cancer should rely on their oncology team; dietary measures are not shown to treat cancer.

## Part 7: Next Steps

1. Characterise the candidate material (source, strain, preparation, composition).
2. Run steps 1–7 in an accredited laboratory with appropriate controls and replicates.
3. Pre-register the falsification criteria above before testing.
4. Update this discovery with results, positive or negative.

## References

- ARC Discovery 126 and its Erratum (Nov 2026); *The Bounded Enzyme Target Hypothesis* (Nov 2026).
- Park MH, Wolff EC. J Biol Chem 2018;293:18710–18718 (hypusine and eIF5A).
- Gutierrez E et al. Mol Cell 2013;51:35–45 (eIF5A and polyproline motifs).
- Jakus J et al. J Biol Chem 1993;268:13151–13159 (GC7 and the DHPS spermidine site).
- Tsherniak A et al. Cell 2017;170:564–576 (DepMap).

---

**THIS IS A HYPOTHESIS FOR LABORATORY TESTING — NOT YET VALIDATED EXPERIMENTALLY.**
