# Validator extesnion

## Available commands:

### list
With this command you can lookup validators for a given epoch. Epoch can be specifyed by EpochId, BlockId, Block hight or `Latest` keyword.
In the terminal command line type:
```txt
./near-validators validators network-config testnet now
```

<details><summary><i>The result of this command will be as follows:</i></summary>

```txt
Validators (total: 90, seat price: 53085.036224843075206029910443 NEAR)
+-------------------------------------------+----------------------------------------+----------+-----------------+-----------------+-----------------+-----------------+
| Validator Id                              | Stake                                  | Online   | Blocks produced | Blocks expected | Chunks produced | Chunks expected |
+-------------------------------------------+----------------------------------------+----------+-----------------+-----------------+-----------------+-----------------+
| node2                                     | 42463456.377087193379078729276997 NEAR |  99.99 % | 5269            | 5271            | 18766           | 18767           |
| node1                                     | 42455461.454552359341703906982801 NEAR |  99.95 % | 5226            | 5230            | 18459           | 18467           |
| node3                                     | 42438161.018506526315943976152203 NEAR |  99.98 % | 5297            | 5299            | 18416           | 18418           |
| node0                                     | 42334729.937985819822434707258555 NEAR | 100.00 % | 5237            | 5237            | 18520           | 18520           |
| aurora.pool.f863973.m0                    | 31177850.233547788240331244407559 NEAR |  99.91 % | 3802            | 3808            | 13358           | 13367           |
| 01node.pool.f863973.m0                    | 15719906.892540506860671479938212 NEAR |  99.98 % | 1891            | 1891            | 6882            | 6884            |
| cryptogarik.pool.f863973.m0               | 14146118.880582007333099210276014 NEAR |   0.00 % | 0               | 1792            | 0               | 6207            |
| legends.pool.f863973.m0                   | 13232045.564115859383036538973294 NEAR |  99.97 % | 1665            | 1665            | 5733            | 5735            |
| stakely_v2.pool.f863973.m0                | 11308412.76413255794143406084197 NEAR  |  99.98 % | 1407            | 1408            | 4762            | 4762            |
| everstake.pool.f863973.m0                 | 11080661.442432259407194895864464 NEAR |  99.95 % | 1400            | 1400            | 4791            | 4794            |
| chorusone.pool.f863973.m0                 | 9629377.891168433658816361005072 NEAR  | 100.00 % | 1201            | 1201            | 4212            | 4212            |
| ni.pool.f863973.m0                        | 8822038.178787812833888556031977 NEAR  |  99.92 % | 1066            | 1067            | 3795            | 3798            |
| kiln.pool.f863973.m0                      | 6079613.843956031425391305083783 NEAR  |  99.88 % | 792             | 792             | 2641            | 2645            |
| staked.pool.f863973.m0                    | 4999757.888727011980090322499078 NEAR  |   0.00 % | 0               | 613             | 0               | 2203            |
| foundryusa.pool.f863973.m0                | 2412323.288338062508844224434456 NEAR  | 100.00 % | 312             | 312             | 1032            | 1032            |
| chorus-one.pool.f863973.m0                | 2219222.836168428864972611997445 NEAR  |  99.84 % | 284             | 284             | 990             | 992             |
| lunanova2.pool.f863973.m0                 | 2122700.547698656819828381670739 NEAR  | 100.00 % | 234             | 234             | 922             | 922             |
| tribe-pool.pool.f863973.m0                | 1602734.408040514998117022773702 NEAR  |   0.00 % | 0               | 204             | 0               | 700             |
| sweden.pool.f863973.m0                    | 1548522.293510958519275997113481 NEAR  |   0.00 % | 0               | 208             | 0               | 691             |
| stakesstone.pool.f863973.m0               | 1495620.398863776083560667101079 NEAR  |  99.62 % | 187             | 187             | 598             | 601             |
| pathrocknetwork.pool.f863973.m0           | 1489110.822070197011527074788129 NEAR  | 100.00 % | 195             | 195             | 663             | 663             |
| bee1stake.pool.f863973.m0                 | 1458170.384325998236160607279491 NEAR  | 100.00 % | 183             | 183             | 654             | 654             |
| alexandruast.pool.f863973.m0              | 1421069.05632136355769699266569 NEAR   |   0.00 % | 0               | 153             | 0               | 641             |
| dsrvlabs.pool.f863973.m0                  | 1360015.07176394889137508520043 NEAR   |  98.94 % | 144             | 145             | 604             | 611             |
| leadnode.pool.f863973.m0                  | 1275260.096422084948847089180765 NEAR  | 100.00 % | 155             | 155             | 543             | 543             |
| basilisk-stake.pool.f863973.m0            | 911982.673163498653723855407971 NEAR   |   0.00 % | 0               | 136             | 0               | 413             |
| namdokmai.pool.f863973.m0                 | 911474.339828150411737273056685 NEAR   |   0.00 % | 0               | 105             | 0               | 355             |
| solidstate.pool.f863973.m0                | 902587.046139049484585657411276 NEAR   |   0.00 % | 0               | 112             | 0               | 393             |
| chelovek_iz_naroda.pool.f863973.m0        | 814633.60270815969401901915999 NEAR    |   0.00 % | 0               | 109             | 0               | 374             |
| optimusvalidatornetwork.pool.f863973.m0   | 814536.394051230138804706693949 NEAR   |   0.00 % | 0               | 93              | 0               | 341             |
| tayang.pool.f863973.m0                    | 737378.959040357990001747402769 NEAR   |   0.00 % | 0               | 80              | 0               | 339             |
| blockngine.pool.f863973.m0                | 639345.96536088610582681658546 NEAR    |   0.00 % | 0               | 79              | 0               | 264             |
| bflame.pool.f863973.m0                    | 604958.879804267092172663214773 NEAR   |   0.00 % | 0               | 62              | 0               | 264             |
| genesislab.pool.f863973.m0                | 577654.162821521726556213824992 NEAR   |   0.00 % | 0               | 66              | 0               | 266             |
| squatch.pool.f863973.m0                   | 553395.136322010026324042602167 NEAR   |   0.00 % | 0               | 74              | 0               | 224             |
| stake2grow.pool.f863973.m0                | 535861.079658259760801920488481 NEAR   |   0.00 % | 0               | 73              | 0               | 211             |
| stgr.pool.f863973.m0                      | 528380.848075029588345471059098 NEAR   |   0.00 % | 0               | 70              | 0               | 233             |
| onin.pool.f863973.m0                      | 524123.873966987768556970647187 NEAR   |   0.00 % | 0               | 68              | 0               | 225             |
| fibocrypto.pool.f863973.m0                | 520520.353322425852655787808568 NEAR   |   0.00 % | 0               | 83              | 0               | 232             |
| anchikovproduction.pool.f863973.m0        | 497897.493465086603547487431967 NEAR   |   0.00 % | 0               | 64              | 0               | 217             |
| darvin.pool.f863973.m0                    | 494852.176425715690951019987015 NEAR   |   0.00 % | 0               | 65              | 0               | 219             |
| moonlet.pool.f863973.m0                   | 480808.594675834324997215741764 NEAR   |   0.00 % | 0               | 57              | 0               | 204             |
| pontiff.pool.f863973.m0                   | 468365.719851475515168941238586 NEAR   |   0.00 % | 0               | 54              | 0               | 195             |
| aquarius.pool.f863973.m0                  | 440148.228029800480983617468605 NEAR   |   0.00 % | 0               | 53              | 0               | 206             |
| casualpooltest.pool.f863973.m0            | 437487.300611972413125697142006 NEAR   |   0.00 % | 0               | 62              | 0               | 198             |
| pennyvalidators.pool.f863973.m0           | 405728.663157549880430291114589 NEAR   |   0.00 % | 0               | 56              | 0               | 175             |
| bazilik.pool.f863973.m0                   | 403050.129599256640576660181764 NEAR   |   0.00 % | 0               | 52              | 0               | 185             |
| mondlicht.pool.devnet                     | 382179.974090138353447112728888 NEAR   | 100.00 % | 45              | 45              | 152             | 152             |
| stingray.pool.f863973.m0                  | 359535.994275044281045666367836 NEAR   |   0.00 % | 0               | 40              | 0               | 154             |
| nw.pool.devnet                            | 314305.897488098565334510551894 NEAR   | 100.00 % | 30              | 30              | 125             | 125             |
| infiniteloop.pool.f863973.m0              | 312813.239752153752739566624169 NEAR   |   0.00 % | 0               | 30              | 0               | 148             |
| gargoyle.pool.f863973.m0                  | 292432.815062289613304478068761 NEAR   | 100.00 % | 29              | 29              | 141             | 141             |
| lastnode.pool.f863973.m0                  | 221061.38599926302676468391151 NEAR    |   0.00 % | 0               | 23              | 0               | 115             |
| gettingnear.pool.f863973.m0               | 205244.215299155470451571795811 NEAR   | 100.00 % | 28              | 28              | 86              | 86              |
| zainy.pool.f863973.m0                     | 196532.135232203214129734163032 NEAR   |   0.00 % | 0               | 32              | 0               | 88              |
| spectrum.pool.f863973.m0                  | 187756.424163657235865226822981 NEAR   |   0.00 % | 0               | 21              | 0               | 77              |
| kronos.pool.f863973.m0                    | 171375.749635294004952803904532 NEAR   |   0.00 % | 0               | 16              | 0               | 67              |
| sashamaxymchuk.pool.f863973.m0            | 152924.123495250792923696161646 NEAR   |   0.00 % | 0               | 17              | 0               | 57              |
| idtcn3.pool.f863973.m0                    | 115576.047411466110628506181867 NEAR   |   0.00 % | 0               | 16              | 0               | 52              |
| sssuryansh.pool.f863973.m0                | 89998.886291308720730791178863 NEAR    |   0.00 % | 0               | 5               | 0               | 30              |
| blueprint.pool.f863973.m0                 | 78978.68796349885502102929427 NEAR     |  98.15 % | 12              | 13              | 41              | 41              |
| hahaha.pool.devnet                        | 64337.498161220467461479588097 NEAR    | 100.00 % | 2               | 2               | 31              | 31              |
| jstaking.pool.f863973.m0                  | 59249.07109749876737048778665 NEAR     |   0.00 % | 0               | 8               | 0               | 16              |
| derori_validator_pool.pool.f863973.m0     | 58645.575112263099871994258981 NEAR    | 100.00 % | 6               | 6               | 26              | 26              |
| ibb.pool.f863973.m0                       | 54704.833517287745250191173298 NEAR    |   0.00 % | 0               | 7               | 0               | 14              |
| happystake.pool.f863973.m0                | 53720.240145927988351697242033 NEAR    |   0.00 % | 0               | 5               | 0               | 25              |
| kuutamo.pool.f863973.m0                   | 50898.649507219560792919189598 NEAR    |   0.00 % | 0               | 0               | 0               | 25              |
| bgpntx.pool.f863973.m0                    | 49788.123993303798255829538717 NEAR    |   0.00 % | 0               | 0               | 0               | 25              |
| grassets.pool.f863973.m0                  | 48754.250378643643185317807387 NEAR    |   0.00 % | 0               | 0               | 0               | 14              |
| pandateam.pool.f863973.m0                 | 47663.661681818850522112907251 NEAR    |   0.00 % | 0               | 0               | 0               | 20              |
| domanodes.pool.f863973.m0                 | 46932.503319601361625002798 NEAR       |   0.00 % | 0               | 0               | 0               | 19              |
| x50capital.pool.f863973.m0                | 45928.257745845634881269534446 NEAR    |   0.00 % | 0               | 0               | 0               | 20              |
| twinstake.pool.f863973.m0                 | 44750.061857222259043729329193 NEAR    | 100.00 % | 0               | 0               | 19              | 19              |
| 4ire-pool.pool.f863973.m0                 | 44394.746192552157237523710969 NEAR    |   0.00 % | 0               | 0               | 0               | 22              |
| twintest1.pool.f863973.m0                 | 43560.75819603592053714114437 NEAR     |   0.00 % | 0               | 0               | 0               | 18              |
| sevennines-t0.pool.f863973.m0             | 43217.387754730002508230464604 NEAR    |   0.00 % | 0               | 0               | 0               | 15              |
| commons_pnw.pool.f863973.m0               | 41307.46845724409836625299375 NEAR     |   0.00 % | 0               | 0               | 0               | 17              |
| cryptolions.pool.f863973.m0               | 38585.308044335751252004590272 NEAR    |   0.00 % | 0               | 0               | 0               | 26              |
| omnistake_v5.factory01.littlefarm.testnet | 38539.722508482341332079252916 NEAR    |   0.00 % | 0               | 0               | 0               | 13              |
| cex2.pool.f863973.m0                      | 37778.83295188551769335374954 NEAR     |   0.00 % | 0               | 0               | 0               | 16              |
| lnc.pool.f863973.m0                       | 37491.208479267156735023643816 NEAR    |   0.00 % | 0               | 0               | 0               | 15              |
| alanpool.pool.f863973.m0                  | 36228.667912753203223689239387 NEAR    |   0.00 % | 0               | 0               | 0               | 15              |
| dialogue.pool.f863973.m0                  | 35963.563252341589944110547084 NEAR    |   0.00 % | 0               | 0               | 0               | 8               |
| stakingpodalliance.pool.f863973.m0        | 35047.110469586773652462433208 NEAR    |   0.00 % | 0               | 0               | 0               | 16              |
| dehashed.pool.f863973.m0                  | 32769.300253705312947757304866 NEAR    |   0.00 % | 0               | 0               | 0               | 8               |
| dav_kuutamo.pool.f863973.m0               | 30330.117372193371695000000001 NEAR    |   0.00 % | 0               | 0               | 0               | 17              |
| lavenderfive.pool.f863973.m0              | 30227.016444935378828600648379 NEAR    |   0.00 % | 0               | 0               | 0               | 11              |
| machfund.pool.f863973.m0                  | 23570.872249580298614866762038 NEAR    |   0.00 % | 0               | 0               | 0               | 16              |
| lusienda.pool.f863973.m0                  | 14635.888149639641051205948527 NEAR    |   0.00 % | 0               | 0               | 0               | 10              |
| gp-validator-testnet.pool.f863973.m0      | 14226.94217859214090210446257 NEAR     |   0.00 % | 0               | 0               | 0               | 4               |
+-------------------------------------------+----------------------------------------+----------+-----------------+-----------------+-----------------+-----------------+
```
</details>

### proposals
Show both new proposals in the current epoch as well as current validators who are implicitly proposing:
```txt
./near-validators proposals network-config testnet
```

<details><summary><i>The result of this command will be as follows:</i></summary>

```txt
Proposals for the epoch after next (new: 25, passing: 62, expected seat price = 54039.777430965844435406680899 NEAR)
+----+--------------------+-------------------------------------------+----------------------------------------+----------------------------------------+
| #  | Status             | Validator Id                              | Stake                                  | New Stake                              |
+----+--------------------+-------------------------------------------+----------------------------------------+----------------------------------------+
| 1  | Rollover           | node2                                     | 42463456.377087193379078729276997 NEAR |                                        |
| 2  | Rollover           | node1                                     | 42455461.454552359341703906982801 NEAR |                                        |
| 3  | Rollover           | node3                                     | 42438161.018506526315943976152203 NEAR |                                        |
| 4  | Rollover           | node0                                     | 42334729.937985819822434707258555 NEAR |                                        |
| 5  | Proposal(Accepted) | aurora.pool.f863973.m0                    | 31177850.233547788240331244407559 NEAR | 31193131.192966929723681068481055 NEAR |
| 6  | Rollover           | 01node.pool.f863973.m0                    | 15719906.892540506860671479938212 NEAR |                                        |
| 7  | Proposal(Accepted) | cryptogarik.pool.f863973.m0               | 14146118.880582007333099210276014 NEAR | 14146130.926226277756099420604526 NEAR |
| 8  | Proposal(Accepted) | legends.pool.f863973.m0                   | 13232045.564115859383036538973294 NEAR | 13238044.769643487245621788479017 NEAR |
| 9  | Proposal(Accepted) | stakely_v2.pool.f863973.m0                | 11308412.76413255794143406084197 NEAR  | 11313710.776257739953163284584824 NEAR |
| 10 | Rollover           | everstake.pool.f863973.m0                 | 11080661.442432259407194895864464 NEAR |                                        |
| 11 | Rollover           | chorusone.pool.f863973.m0                 | 9629377.891168433658816361005072 NEAR  |                                        |
| 12 | Proposal(Accepted) | ni.pool.f863973.m0                        | 8822038.178787812833888556031977 NEAR  | 8825232.420561305823830133509952 NEAR  |
| 13 | Proposal(Accepted) | nodeasy.pool.f863973.m0                   |                                        | 7934172.945372108536470046666193 NEAR  |
| 14 | Proposal(Accepted) | kiln.pool.f863973.m0                      | 6079613.843956031425391305083783 NEAR  | 6089293.096542765174318484627714 NEAR  |
| 15 | Rollover           | staked.pool.f863973.m0                    | 4999757.888727011980090322499078 NEAR  |                                        |
| 16 | Rollover           | foundryusa.pool.f863973.m0                | 2412323.288338062508844224434456 NEAR  |                                        |
| 17 | Rollover           | chorus-one.pool.f863973.m0                | 2219222.836168428864972611997445 NEAR  |                                        |
| 18 | Rollover           | lunanova2.pool.f863973.m0                 | 2122700.547698656819828381670739 NEAR  |                                        |
| 19 | Rollover           | tribe-pool.pool.f863973.m0                | 1602734.408040514998117022773702 NEAR  |                                        |
| 20 | Rollover           | sweden.pool.f863973.m0                    | 1548522.293510958519275997113481 NEAR  |                                        |
| 21 | Proposal(Accepted) | stakesstone.pool.f863973.m0               | 1495620.398863776083560667101079 NEAR  | 1496166.078305000144619938927897 NEAR  |
| 22 | Proposal(Accepted) | pathrocknetwork.pool.f863973.m0           | 1489110.822070197011527074788129 NEAR  | 1489649.148331873661555724498084 NEAR  |
| 23 | Rollover           | bee1stake.pool.f863973.m0                 | 1458170.384325998236160607279491 NEAR  |                                        |
| 24 | Rollover           | alexandruast.pool.f863973.m0              | 1421069.05632136355769699266569 NEAR   |                                        |
| 25 | Rollover           | dsrvlabs.pool.f863973.m0                  | 1360015.07176394889137508520043 NEAR   |                                        |
| 26 | Rollover           | leadnode.pool.f863973.m0                  | 1275260.096422084948847089180765 NEAR  |                                        |
| 27 | Rollover           | namdokmai.pool.f863973.m0                 | 911474.339828150411737273056685 NEAR   |                                        |
| 28 | Rollover           | solidstate.pool.f863973.m0                | 902587.046139049484585657411276 NEAR   |                                        |
| 29 | Proposal(Accepted) | chelovek_iz_naroda.pool.f863973.m0        | 814633.60270815969401901915999 NEAR    | 814643.602843622897090819159989 NEAR   |
| 30 | Proposal(Accepted) | optimusvalidatornetwork.pool.f863973.m0   | 814536.394051230138804706693949 NEAR   | 814525.597100869446858861876735 NEAR   |
| 31 | Rollover           | tayang.pool.f863973.m0                    | 737378.959040357990001747402769 NEAR   |                                        |
| 32 | Rollover           | blockngine.pool.f863973.m0                | 639345.96536088610582681658546 NEAR    |                                        |
| 33 | Rollover           | bflame.pool.f863973.m0                    | 604958.879804267092172663214773 NEAR   |                                        |
| 34 | Rollover           | genesislab.pool.f863973.m0                | 577654.162821521726556213824992 NEAR   |                                        |
| 35 | Rollover           | squatch.pool.f863973.m0                   | 553395.136322010026324042602167 NEAR   |                                        |
| 36 | Rollover           | stake2grow.pool.f863973.m0                | 535861.079658259760801920488481 NEAR   |                                        |
| 37 | Rollover           | onin.pool.f863973.m0                      | 524123.873966987768556970647187 NEAR   |                                        |
| 38 | Rollover           | fibocrypto.pool.f863973.m0                | 520520.353322425852655787808568 NEAR   |                                        |
| 39 | Rollover           | anchikovproduction.pool.f863973.m0        | 497897.493465086603547487431967 NEAR   |                                        |
| 40 | Rollover           | darvin.pool.f863973.m0                    | 494852.176425715690951019987015 NEAR   |                                        |
| 41 | Proposal(Accepted) | infstones.pool.f863973.m0                 |                                        | 490042.289162263103709480311607 NEAR   |
| 42 | Rollover           | moonlet.pool.f863973.m0                   | 480808.594675834324997215741764 NEAR   |                                        |
| 43 | Rollover           | aquarius.pool.f863973.m0                  | 440148.228029800480983617468605 NEAR   |                                        |
| 44 | Proposal(Accepted) | casualpooltest.pool.f863973.m0            | 437487.300611972413125697142006 NEAR   | 437487.304290901270779497142006 NEAR   |
| 45 | Rollover           | pennyvalidators.pool.f863973.m0           | 405728.663157549880430291114589 NEAR   |                                        |
| 46 | Proposal(Accepted) | mondlicht.pool.devnet                     | 382179.974090138353447112728888 NEAR   | 382518.134699398818830702935521 NEAR   |
| 47 | Rollover           | stingray.pool.f863973.m0                  | 359535.994275044281045666367836 NEAR   |                                        |
| 48 | Rollover           | nw.pool.devnet                            | 314305.897488098565334510551894 NEAR   |                                        |
| 49 | Proposal(Accepted) | infiniteloop.pool.f863973.m0              | 312813.239752153752739566624169 NEAR   | 312813.240053274445572066624169 NEAR   |
| 50 | Rollover           | gargoyle.pool.f863973.m0                  | 292432.815062289613304478068761 NEAR   |                                        |
| 51 | Proposal(Accepted) | lastnode.pool.f863973.m0                  | 221061.38599926302676468391151 NEAR    | 221061.38627961502993308391151 NEAR    |
| 52 | Proposal(Accepted) | gettingnear.pool.f863973.m0               | 205244.215299155470451571795811 NEAR   | 205828.309465759993436213346325 NEAR   |
| 53 | Rollover           | spectrum.pool.f863973.m0                  | 187756.424163657235865226822981 NEAR   |                                        |
| 54 | Rollover           | kronos.pool.f863973.m0                    | 171375.749635294004952803904532 NEAR   |                                        |
| 55 | Rollover           | idtcn3.pool.f863973.m0                    | 115576.047411466110628506181867 NEAR   |                                        |
| 56 | Proposal(Accepted) | kuutamocharlie.pool.devnet                |                                        | 81955.191886364504871018375552 NEAR    |
| 57 | Rollover           | blueprint.pool.f863973.m0                 | 78978.68796349885502102929427 NEAR     |                                        |
| 58 | Rollover           | hahaha.pool.devnet                        | 64337.498161220467461479588097 NEAR    |                                        |
| 59 | Proposal(Accepted) | forked.pool.f863973.m0                    |                                        | 60212.05554749766575529530327 NEAR     |
| 60 | Rollover           | jstaking.pool.f863973.m0                  | 59249.07109749876737048778665 NEAR     |                                        |
| 61 | Rollover           | derori_validator_pool.pool.f863973.m0     | 58645.575112263099871994258981 NEAR    |                                        |
| 62 | Rollover           | ibb.pool.f863973.m0                       | 54704.833517287745250191173298 NEAR    |                                        |
| 63 | Kicked out         | happystake.pool.f863973.m0                | 53720.240145927988351697242033 NEAR    |                                        |
| 64 | Kicked out         | kuutamo.pool.f863973.m0                   | 50898.649507219560792919189598 NEAR    |                                        |
| 65 | Proposal(Declined) | bgpntx.pool.f863973.m0                    | 49788.123993303798255829538717 NEAR    | 49788.124271479370135129538717 NEAR    |
| 66 | Kicked out         | grassets.pool.f863973.m0                  | 48754.250378643643185317807387 NEAR    |                                        |
| 67 | Kicked out         | pandateam.pool.f863973.m0                 | 47663.661681818850522112907251 NEAR    |                                        |
| 68 | Kicked out         | domanodes.pool.f863973.m0                 | 46932.503319601361625002798 NEAR       |                                        |
| 69 | Kicked out         | x50capital.pool.f863973.m0                | 45928.257745845634881269534446 NEAR    |                                        |
| 70 | Kicked out         | twinstake.pool.f863973.m0                 | 44750.061857222259043729329193 NEAR    |                                        |
| 71 | Kicked out         | 4ire-pool.pool.f863973.m0                 | 44394.746192552157237523710969 NEAR    |                                        |
| 72 | Kicked out         | twintest1.pool.f863973.m0                 | 43560.75819603592053714114437 NEAR     |                                        |
| 73 | Kicked out         | sevennines-t0.pool.f863973.m0             | 43217.387754730002508230464604 NEAR    |                                        |
| 74 | Kicked out         | commons_pnw.pool.f863973.m0               | 41307.46845724409836625299375 NEAR     |                                        |
| 75 | Proposal(Declined) | cryptolions.pool.f863973.m0               | 38585.308044335751252004590272 NEAR    | 38585.308374159098843004590272 NEAR    |
| 76 | Kicked out         | omnistake_v5.factory01.littlefarm.testnet | 38539.722508482341332079252916 NEAR    |                                        |
| 77 | Kicked out         | cex2.pool.f863973.m0                      | 37778.83295188551769335374954 NEAR     |                                        |
| 78 | Kicked out         | lnc.pool.f863973.m0                       | 37491.208479267156735023643816 NEAR    |                                        |
| 79 | Kicked out         | alanpool.pool.f863973.m0                  | 36228.667912753203223689239387 NEAR    |                                        |
| 80 | Kicked out         | dialogue.pool.f863973.m0                  | 35963.563252341589944110547084 NEAR    |                                        |
| 81 | Kicked out         | stakingpodalliance.pool.f863973.m0        | 35047.110469586773652462433208 NEAR    |                                        |
| 82 | Kicked out         | dehashed.pool.f863973.m0                  | 32769.300253705312947757304866 NEAR    |                                        |
| 83 | Proposal(Declined) | do0k13-kuutamod.pool.devnet               |                                        | 31893.204026221938212322781368 NEAR    |
| 84 | Kicked out         | dav_kuutamo.pool.f863973.m0               | 30330.117372193371695000000001 NEAR    |                                        |
| 85 | Proposal(Declined) | lavenderfive.pool.f863973.m0              | 30227.016444935378828600648379 NEAR    | 30227.016817078602784800648379 NEAR    |
| 86 | Kicked out         | machfund.pool.f863973.m0                  | 23570.872249580298614866762038 NEAR    |                                        |
| 87 | Kicked out         | lusienda.pool.f863973.m0                  | 14635.888149639641051205948527 NEAR    |                                        |
| 88 | Proposal(Declined) | 1inc4.pool.f863973.m0                     |                                        | 8970.16910365545105495283601 NEAR      |
| 89 | Proposal(Declined) | wolfedge-capital-testnet.pool.f863973.m0  |                                        | 4110.352445422739638628282042 NEAR     |
+----+--------------------+-------------------------------------------+----------------------------------------+----------------------------------------+
```
</details>
