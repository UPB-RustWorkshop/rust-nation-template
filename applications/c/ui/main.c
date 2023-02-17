#include "lvgl_driver.h"
#include <stdio.h>
#include <timer.h>

typedef struct {
  bool door1;
  bool door2;
} doors_t;

static doors_t get_doors(void) {
  syscall_return_t res = command(0xa0000, 201, 0, 0);
  doors_t doors = { res.data[0] != 0, res.data[1] != 0 };
  return doors;
}

static int get_temperarture(void) {
  syscall_return_t res = command(0xa0000, 202, 0, 0);
  return res.data[0];
}

// static void event_handler(lv_event_t * e)
// {
//   lv_event_code_t code  = lv_event_get_code(e);
//   unsigned int *seconds = (unsigned int*)lv_event_get_user_data(e);

//   if (code == LV_EVENT_CLICKED) {
//     LV_LOG_USER("Clicked");
//     *seconds = 0;
//   } else if (code == LV_EVENT_VALUE_CHANGED) {
//     LV_LOG_USER("Toggled");
//   }
// }

int main (void)
{
  unsigned int seconds = 0;

  screen_set_brightness(100);
  int status = lvgl_driver_init(5);
  if (status == RETURNCODE_SUCCESS) {
    /* LittlevGL's Hello World tutorial example */

    lv_obj_t * scr = lv_disp_get_scr_act(NULL);         /*Get the current screen*/

    /*Create a Label on the currently active screen*/
    lv_obj_t * temperature_label =  lv_label_create(scr);

    /*Modify the Label's text*/
    lv_label_set_text(temperature_label, "Loading");

    /* Align the Label to the center
     * NULL means align on parent (which is the screen now)
     * 0, 0 at the end means an x, y offset after alignment*/
    lv_obj_align(temperature_label, LV_ALIGN_CENTER, 0, 70);

    /*Create a Label on the currently active screen*/
    lv_obj_t * doors_label =  lv_label_create(scr);

    /*Modify the Label's text*/
    lv_label_set_text(doors_label, "");

    /* Align the Label to the center
     * NULL means align on parent (which is the screen now)
     * 0, 0 at the end means an x, y offset after alignment*/
    lv_obj_align(doors_label, LV_ALIGN_CENTER, 0, 90);

    // lv_obj_t * btn1 = lv_btn_create(scr);
    // lv_obj_add_event_cb(btn1, event_handler, LV_EVENT_ALL, &seconds);
    // lv_obj_align(btn1, LV_ALIGN_CENTER, 0, -40);

    // lv_obj_t * label = lv_label_create(btn1);
    // lv_label_set_text(label, "Reset");

    // Temperature
    lv_obj_t * meter = lv_meter_create(lv_scr_act());
    lv_obj_set_pos(meter, 10, 10);
    lv_obj_set_size(meter, 150, 150);

    /*Add a scale first*/
    lv_meter_scale_t * scale = lv_meter_add_scale(meter);
    lv_meter_set_scale_ticks(meter, scale, 30, 2, 10, lv_palette_main(LV_PALETTE_GREY));
    lv_meter_set_scale_major_ticks(meter, scale, 5, 4, 10, lv_color_black(), 10);

    lv_meter_set_scale_range(meter, scale, -10, 50, 300, -240);

    lv_meter_indicator_t * indic;

    /*Add a blue arc to the start*/
    indic = lv_meter_add_arc(meter, scale, 3, lv_palette_main(LV_PALETTE_BLUE), 0);
    lv_meter_set_indicator_start_value(meter, indic, -10);
    lv_meter_set_indicator_end_value(meter, indic, 0);

    /*Make the tick lines blue at the start of the scale*/
    indic = lv_meter_add_scale_lines(meter, scale, lv_palette_main(LV_PALETTE_BLUE), lv_palette_main(LV_PALETTE_BLUE), false, 0);
    lv_meter_set_indicator_start_value(meter, indic, -10);
    lv_meter_set_indicator_end_value(meter, indic, 0);

    /*Add a red arc to the end*/
    indic = lv_meter_add_arc(meter, scale, 3, lv_palette_main(LV_PALETTE_RED), 0);
    lv_meter_set_indicator_start_value(meter, indic, 30);
    lv_meter_set_indicator_end_value(meter, indic, 50);

    /*Make the tick lines red at the end of the scale*/
    indic = lv_meter_add_scale_lines(meter, scale, lv_palette_main(LV_PALETTE_RED), lv_palette_main(LV_PALETTE_RED), false, 0);
    lv_meter_set_indicator_start_value(meter, indic, 30);
    lv_meter_set_indicator_end_value(meter, indic, 50);

    /*Add a needle line indicator*/
    indic = lv_meter_add_needle_line(meter, scale, 4, lv_palette_main(LV_PALETTE_GREY), -10);



    /* main loop */
    while (1) {
      seconds++;
      if (seconds % 200 == 0) {
        char buffer[100];
        int temperature = get_temperarture() / 100;
        doors_t doors = get_doors();
        snprintf(buffer, 99, "Temperature: %d", temperature);
        lv_meter_set_indicator_value(meter, indic, temperature);
        lv_label_set_text(temperature_label, buffer);
        snprintf(buffer, 99, "Door1: %s, Door2: %s", doors.door1?"Closed":"Open", doors.door2?"Closed":"Open");
        lv_label_set_text(doors_label, buffer);
      }
      delay_ms(5);
      lvgl_driver_event(5);
    }
  } else {
    printf("lvgl init error: %s\n", tock_strrcode(status));
  }
  return 0;
}
