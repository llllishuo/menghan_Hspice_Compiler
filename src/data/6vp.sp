*------------1vp--------------------
.option post = 2
.option search ='./'
.lib 'crn018_rfid_v1d1.l'tt
.subckt zhediegongyuangongshan agnd avdd ib vin vip vo
    m27 ib ib agnd agnd nch l=180n w=2u m=1         
    m20 net35 vbn1 agnd agnd nch l=3u w=4u m=1      
    m11 net38 vbn2 agnd agnd nch l=3u w=4u m=1   
    m21 vbp2 ib agnd agnd nch l=180n w=2u m=1   
    m18 vbn1 vbn2 net35 agnd nch l=1u w=4u m=1 
    m22 vbp1 ib agnd agnd nch l=180n w=2u m=1 
    m16 vbn2 vbn2 net38 agnd nch l=1u w=4u m=1 
    m6 net30 vip net14 agnd nch l=180n w=2u m=1 
    m5 net31 vin net14 agnd nch l=180n w=2u m=1 
    m4 net14 vbn1 agnd agnd nch l=3u w=4u m=1 
    m3 net46 net17 agnd agnd nch l=3u w=4u m=1 
    m2 net47 net17 agnd agnd nch l=3u w=4u m=1 
    m1 vo vbn2 net46 agnd nch l=1u w=4u m=1 
    m0 net17 vbn2 net47 agnd nch l=1u w=4u m=1 
    m26 vbp1 vbp2 net33 avdd pch l=1u w=8u m=1 
    m25 vbp2 vbp2 net34 avdd pch l=1u w=8u m=1
    m24 net33 vbp1 avdd avdd pch l=3u w=8u m=1 
    m23 net34 vbp2 avdd avdd pch l=3u w=8u m=1 
    m15 vbn1 vbp2 net36 avdd pch l=1u w=8u m=1 
    m14 vbn2 vbp2 net37 avdd pch l=1u w=8u m=1 
    m13 net36 vbp1 avdd avdd pch l=3u w=8u m=1 
    m12 net37 vbp1 avdd avdd pch l=3u w=8u m=1 
    m10 net17 vbp2 net30 avdd pch l=1u w=8u m=1 
    m9 net30 vbp1 avdd avdd pch l=3u w=8u m=1 
    m8 net31 vbp1 avdd avdd pch l=3u w=8u m=1 
    m7 vo vbp2 net31 avdd pch l=1u w=8u m=1
.ends 

xi5 agnd avdd ib vin vip vo vo zhediegongyuangongshan
r1 agnd 0 0k
v0 avdd 0 1.8
ib ib 0 3.2u
v1 vin 0 0.9
v2 vip 0 0.9
.end

