*------------1vp--------------------
.option post = 2
.option search ='./'
.lib 'crn018_rfid_v1d1.l'tt

.global vdd gnd 

.param lx=0.1p 

.subckt inv n1 n2
m1 n2 n1 vdd vdd pch l=1u w=6.6u
m2 n2 n1 gnd gnd nch l=1u w=1.5u
.ends

X1 n1 n2 inv 
v1 vdd gnd 5
v2 n1 gnd pulse 0 5 10n 0.2n 0.2n 10n 20n
c1 n2 gnd lx

.tran 0.1n 1000n sweep lx 0.1p 0.2p 0.1p 
.meas tran xj trig V (n1)val = 2.5 fall=1 targ V (n2)val = 2.5 rise=2
.meas tran ss trig V (n1)val = 2.5 rise=1 targ V (n2)val = 2.5 fall=2
.meas pj param='(ss+xj)/2'
.meas tran sj trig V (n2)val = 0.5 rise=1 targ V (n2)val = 2.5 rise=2
.meas tran xh trig V (n2)val = 4.5 fall=1 targ V (n2)val = 2.5 fall=2

.end 
