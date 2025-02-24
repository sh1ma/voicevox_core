package jp.hiroshiba.voicevoxcore;

import com.google.gson.Gson;
import com.google.gson.annotations.Expose;
import com.google.gson.annotations.SerializedName;
import jakarta.annotation.Nonnull;

/** 音声モデル。 */
public class VoiceModel extends Dll {
  private long handle;

  /** ID。 */
  @Nonnull public final String id;

  /** メタ情報。 */
  @Nonnull public final SpeakerMeta[] metas;

  public VoiceModel(String modelPath) {
    rsFromPath(modelPath);
    id = rsGetId();
    String metasJson = rsGetMetasJson();
    Gson gson = new Gson();
    SpeakerMeta[] rawMetas = gson.fromJson(metasJson, SpeakerMeta[].class);
    if (rawMetas == null) {
      throw new RuntimeException("Failed to parse metasJson");
    }
    metas = rawMetas;
  }

  protected void finalize() throws Throwable {
    rsDrop();
    super.finalize();
  }

  private native void rsFromPath(String modelPath);

  @Nonnull
  private native String rsGetId();

  @Nonnull
  private native String rsGetMetasJson();

  private native void rsDrop();

  /** 話者（speaker）のメタ情報。 */
  public static class SpeakerMeta {
    /** 話者名。 */
    @SerializedName("name")
    @Expose
    @Nonnull
    public final String name;

    /** 話者に属するスタイル。 */
    @SerializedName("styles")
    @Expose
    @Nonnull
    public final StyleMeta[] styles;

    /** 話者のUUID。 */
    @SerializedName("speaker_uuid")
    @Expose
    @Nonnull
    public final String speakerUuid;

    /** 話者のバージョン。 */
    @SerializedName("version")
    @Expose
    @Nonnull
    public final String version;

    private SpeakerMeta() {
      // GSONからコンストラクトするため、このメソッドは呼ばれることは無い。
      // このメソッドは@Nonnullを満たすために必要。
      this.name = "";
      this.styles = new StyleMeta[0];
      this.speakerUuid = "";
      this.version = "";
    }
  }

  /** スタイル（style）のメタ情報。 */
  public static class StyleMeta {
    /** スタイル名。 */
    @SerializedName("name")
    @Expose
    @Nonnull
    public final String name;

    /** スタイルID。 */
    @SerializedName("id")
    @Expose
    public final int id;

    private StyleMeta() {
      this.name = "";
      this.id = 0;
    }
  }
}
