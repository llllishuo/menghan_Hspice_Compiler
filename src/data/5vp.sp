*------------2vp--------------------
.option post = 2
.option search ='./'
.lib 'crn018_rfid_v1d1.l'tt
.lib 'crn018_rfid_v1d1.l'TT_BIP
.lib'crn018_rfid_v1d1.l' TT_RES 
m1 n1 n1 n3 gnd nch w=30u l=6u m=3
m2 n2 n1 n4 gnd nch w=30u l=6u m=16
m3 n1 n2 vdd vdd pch w=30u l=6u m=3
m4 n2 n2 vdd vdd pch w=30u l=6u m=3
m5 vo n2 vdd vdd pch w=30u l=6u m=3
q1 gnd gnd n3 pnp10 m=1
q2 gnd gnd n5 pnp10 m=8
q3 gnd gnd n6 pnp10 m=1
xr1 n4 n5 rnpo1rpo w=2u l=27.84u m=1
xr2 vo n6 rnwsti w=2u l=15.52u m=1

v1 vdd gnd 3
.dc v1 0 5.5 0.01
.dc temp -40 120 5
.meas dc XVMAX max v(vo) from =-40 to 120
.meas dc XVmin min v(vo) from =-40 to 120
.meas dc XVavg avg v(vo) from =-40 to 120
.alter fast-fast
.lib 'crn018_rfid_v1d1.l'ss 
.lib 'crn018_rfid_v1d1.l'SS_BIP
.lib'crn018_rfid_v1d1.l' SS_RES 
.alter slow-slow
.lib 'crn018_rfid_v1d1.l'FF
.lib 'crn018_rfid_v1d1.l'FF_BIP
.lib'crn018_rfid_v1d1.l' FF_RES 
.end
