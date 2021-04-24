use tui::layout::Rect;

/// Application banners that ordered from small to big.
const BANNERS: &[&str] = &[
	env!("CARGO_PKG_NAME"),
	r#"
 .ydhh/ +mdhh: :hddy.
 sm` ym +m- sd`hy  ys
 sm`--- /h-`sy`yy`:--
 oh`/sh /hsso- hy./mh
 +h-`sy /h.    hd.-my
  :oo+. -o.    `+sy+`-tui
"#,
	r#"
 -sdmmh+`  sddddho.   /hmmdy:
 :mms:/dmh  ymd//smm: smm+:+hh:
 +mm`  omm` ymh   mdo yho   yds
 +mm`  ```  ohs   hh+ sho   ```
 +md`/ssss  ohy::+hh: hmy yysyo
 /hh`-/shy` ohhyyyo-  yhs //mms
 /hh`  +hy` ohs       yds   dms
 .yho:/yho  sdy       omd+/smm/
  .+syyo:`  +so        -shdhs- 
 `----------` `..... ..  .. `.`
 :ssssssssss- /ydmys dd  dd /m/
                ym`  hd..dd /m/
                oh`  .shhs. :h:
"#,
	r#"
   `/shddhs:    .hhhhhhyo:      :shdddy/`
  -ddmmddmmmh`  .mmmmmmmmmd-  `ymmmddmmdh-
  hmmh.  .dmms  .mmm/``.ymmm` ommm-  `ohhy
  dmms    hmmy  .mmm:   :mmm. odhs`   /hhd`
  dmms    -//-  .hhh:   -hhh. +hhs`   .:/:
  dmms ```````  .hhh:   :hhh. +hhy``...```
  dmd+ +hhhhho  .hhho//ohhho` smmd`/ddhhmm`
  hhh+ :ooyhho  .hhhhhhddh/`  oddy`.ooymmm`
  yhh+    shho  .hhh/--.`     +hhy`   ommm`
  yhho    shho  .hhh:         odmd`   ommm`
  /hhh+::ohhh:  .ddd:         :mmms:/ommms
   :yhhhhhhs:   .hhh:          -sdmmmmmd+`
  `.-:+oo+:-....-///. ........`.``:/o+-...
  ommmmmmmmmmmmmmmmm/ mmmmmmmyymy  -mm-smy
   `````````````````  ``/mm.``ymy  -mm-smh
                        :mm`  omd:-smm.smh
                        :mm`   +hmmds- smy
                         ``       `     ``
"#,
	r#"
   /ydmmdy:    ddddddho-    .sdmmmh+`                       
  ymmdssdmmy   mmmyyymmmo  :mmmysydhy.                      
 -mmd`  `mmm-  mmm.  `mmm. hmm/   +hho                      
 -mmd`   hhh-  mmd.   hdh- shh:   :yho                      
 -mmd`         hhh.   yhh- shh:                             
 -mmh`:sssss.  hhh:--+hhy` ymm/`hyysy+ /yyyyyyyy+           
 -dhy`:sshhh-  hhhhhhdds.  hdd:`sshmms +dddddddds           
 -hhy`   yhh-  hhh/::-`    shh:   omms  :yyyyyy/ss.  ss`oy- 
 -hhy`   hhh.  hhh.        ymm/   omms  ./+mmo/-mm-  mm-hm/ 
  shhs//shhs`  ddd.        /mmhoosmmm:     mm.  dm/ -mm.hm/ 
   :syhhys/`   yyy.         .odmmmds.      mm.  -ymmmh/ hm/ 
      ```                      ```         ``     ```   ``` 
"#,
	r#"
      `-:/:.        ........`           `-:/:.
    +hmmmmmmms-     mmmmmmmmmds-     `+dmmmmmmms.
  `hmdmmhhdmmmm+    mmmmmddmmmmmo   .dmmmmhhdmddh:
  ommmh.   :mmmm.   mmmm-```:dmmm+  smmmy.   :hhhy`
  ymmmo     mmmm:   mmmm-    ommms  yddh/     hhhd-
  ymmmo     yhhh-   mddd-    +hhho  shhh/     syyh.
  ymmmo             hhhh-    +hhho  shhh/
  ymmmo .:::::::`   hhhh-   `shhh+  shdd+ .////::/` `------------
  ymdh+ :hhhhhhh-   hhhhsoooyhhhs.  hmmm+ /dhhhdmm- /mmmmmmmmmmmh`
  shhh+ -osshhhh-   hhhhhhhddmd+`   yddh/ -ossmmmm- :dddddddddddy`
  ohhh+     yhhh-   hhhh+///:.`     shhh/     mmmm-   ossssssss-oo+   .oo:`oo+
  ohhh+     yhhh-   hhhh-           shdd+     mmmm-   yddmmmddd/mmd`  :mmo`mmh`
  +hhhs`   -hhhh-   hhhh-           smmmy`   :mmmm.      ymm-   mmd`  :mmo`mmh`
  `shhhyooshhhh+`   ddmm-           .ddddhsydmmmm+       ymm-   dmm: `smm+`mmh`
   `/yhhhhhhhs:`    hhhh-            `/ymmmmmmmy:        ymm-   .hmmmmmms``mmh`
      .://:-.       .---`               ./++/:`          :++.     ./++:.   ++/
"#,
	r#"
       -+osyys+-         ++++++++++:.            `/osyyso:`
    `+dmdmmmmmmmmo`      mmmmmmmmmmmmms-       :hmmmmmmmmmmy-
   .dmdmmmmdmmmmmmm-     mmmmmmmmmmmmmmmo     smmmmmmmmmmmddd+
   hmmmmh/.``-ymmmmm.    mmmmm/---:ommmmmo   /mmmmmo.``./yhhhh/
  `mmmmm-      mmmmm:    mmmmm:     /mmmmd`  ymmmmo`     +hhhho`
  `mmmmm-      mmmmm/    mmmmm:     -mmmmd.  shhhh+      /hhhdy`
  `mmmmm-      syyyy:    ddddd-     -hhhhy.  ohhhh+      :syyyo`
  `mmmmm-               `hhhhh-     -hhhhy.  ohhhh+
  `mmmmm-               `hhhhh-     :hhhhy.  ohhhh+
  `mmmmd. .hhhhhhhhh:   `hhhhh:```./yhhhho`  sdmmmo  hmmddhhdmy  -hhhhhhhhhhhhhhh`
  `mmhhh. .hhhhhhhhh:   `hhhhhhhhhhhhhdds.   ymmmmo  yhhhhhmmmy` -mmmmmmmmmmmmmmm.
  `hhhhh. `oooohhhhh:   `hhhhhhhhhdmmmy:`    sdhhh+  /ooohmmmmy` -hhhhhhhhhhhhhhh.
  `hhhhh.      yhhhh:   `hhhhho++++/-.`      ohhhh+      ommmmy`   -ooooooooooo.///-    -//: .+++.
  `hhhhh.      yhhhh:   `hhhhh-              ohhhh+      ommmmy`   :mmmmmmmmmmm-mmms`   smmd`:mmm:
  `hhhhh-      yhhhh:   `hhhhh-              smmmmo      ommmmy`   `///smmmo///.mmmy`   smmd`:mmm:
   shhhhs.   `+hhhhy.   `hhhhh-              +mmmmm/   `/mmmmm+        /mmm:    mmmy`   smmd`:mmm:
   .yhhhhhyyyhhhhhy/`    dmmmm:               sdddhhhddmmmmmmy.        /mmm:    hmmd:``:dmmy`:mmm:
    `+yhhhhhhhhhyo-     `hhhhh-                :shdmmmmmmmmh/`         /mmm:    .hmmmmmmmmh- :mmm:
      `-+ossso+:.`       /++++.                  -+syhhys+-`           :hhh-      -oyhhyo:`  -hhh:
"#,
];

/// Application banner.
#[derive(Debug)]
pub struct Banner;

impl Banner {
	/// Returns the appropriate sized banner based on the given dimensions.
	pub fn get(mut rect: Rect) -> String {
		rect.height = rect.height.checked_sub(4).unwrap_or(rect.height);
		format!(
			"{}\n\
            {} ({})\n\
            Author: {}\n\
            Homepage: {}",
			BANNERS
				.iter()
				.rev()
				.find(|banner| {
					usize::from(rect.height) > banner.lines().count()
						&& usize::from(rect.width)
							> banner
								.lines()
								.max_by(|x, y| x.len().cmp(&y.len()))
								.unwrap_or_default()
								.len()
				})
				.unwrap_or(&BANNERS[0]),
			env!("CARGO_PKG_DESCRIPTION"),
			env!("CARGO_PKG_VERSION"),
			env!("CARGO_PKG_AUTHORS"),
			env!("CARGO_PKG_HOMEPAGE")
		)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_app_banner() {
		assert!(Banner::get(Rect {
			x: 0,
			y: 0,
			width: 50,
			height: 30,
		})
		.contains(BANNERS[3]));
		assert!(Banner::get(Rect {
			x: 0,
			y: 0,
			width: 85,
			height: 30,
		})
		.contains(BANNERS[5]))
	}
}
