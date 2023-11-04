*----------1vp------------
.option post = 2
.option search ='./'
.lib 'crn018_rfid_v1d1.l'tt
.subckt zdgygs agnd avdd ib vin vip vo

m6 net207 vip net195 agnd nch l=180n w=2u m=1 
m5 net208 vin net195 agnd nch l=180n w=2u m=1 
m18 vbn1 vbn2 net213 agnd nch l=1u w=4u m=1 
m16 vbn2 vbn2 net211 agnd nch l=1u w=4u 
m1 vo vbn2 net209 agnd nch l=1u w=4u m=1
m0 net197 vbn2 net210 agnd nch l=1u w=4u 
m27 ib ib agnd agnd nch l=4u w=2u m=1 
m20 net213 vbn1 agnd agnd nch l=3u w=4u m=1 
m11 net211 vbn2 agnd agnd nch l=8u w=1u m=1 
m21 vbp2 ib agnd agnd nch l=4u w=2u m=1 
m22 vbp1 ib agnd agnd nch l=4u w=2u m=1 
m4 net195 vbn1 agnd agnd nch l=3u w=4u m=8 
m3 net209 net197 agnd agnd nch l=3u w=4u m=2
m2 net210 net197 agnd agnd nch l=3u w=4u m=2
m24 net216 vbp1 avdd avdd pch l=3u w=8u m=1 
m23 net215 vbp2 avdd avdd pch l=5u w=2u m=1 
m13 net214 vbp1 avdd avdd pch l=3u w=8u m=1 
m12 net212 vbp1 avdd avdd pch l=3u w=8u m=1 
m9 net207 vbp1 avdd avdd pch l=3u w=8u m=6 
m8 net208 vbp1 avdd avdd pch l=3u w=8u m=6 
m26 vbp1 vbp2 net216 avdd pch l=1u w=8u m=1 
m25 vbp2 vbp2 net215 avdd pch l=1u w=8u m=1 
m15 vbn1 vbp2 net214 avdd pch l=1u w=8u m=1 
m14 vbn2 vbp2 net212 avdd pch l=1u w=8u m=1
m10 net197 vbp2 net207 avdd pch l=1u w=8u m=2 
m7 vo vbp2 net208 avdd pch l=1u w=8u m=2 
.ends 

x1 0 avdd ib vin vip vo zdgygs
r1 agnd 0 0k
v0 avdd 0 1.8
ib avdd ib 2u
v1 vip 0 dc 900m ac 1 sin 0 0 1k
c1 vin 0 1G 
l1 vin vo 1G 
c2 vo 0 1p 
.ac dec 10 1 1000MEG 
.probe ac v(vo)
.end

